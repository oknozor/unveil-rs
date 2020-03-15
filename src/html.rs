use anyhow::Result;
use horrorshow::{helper::doctype, prelude::*};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use sass_rs::Options as SassOption;

// from zola https://github.com/getzola/zola/blob/1972e58823417a58eb1cc646ee346e7c3b04addb/components/front_matter/src/lib.rs
lazy_static! {
    static ref PAGE_RE: Regex =
        Regex::new(r"^[[:space:]]*\+\+\+\r?\n((?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$").unwrap();
}

pub struct Preprocessor {
    pub html: String,
    pub scss: String,
    pub markdown: Vec<String>,
    live_reload: bool,
}

impl Preprocessor {
    pub fn build(&mut self) -> Result<(Option<String>, String)> {
        self.markdown_to_html();
        self.insert_user_class();
        self.insert_playpen_button();

        let css = if !self.scss.is_empty() {
            Some(
                sass_rs::compile_string(&self.scss, SassOption::default())
                    .map_err(|err| anyhow!("Unable to compile style matter : {}", err))?,
            )
        } else {
            None
        };

        let has_user_css = css.as_ref().is_some();
        let html = html! {
            : doctype::HTML;
            html(lang="EN") {
                head {
                    meta(charset="utf8");
                    title : "Unveil";
                    link(rel="stylesheet", href="unveil.css");
                    |tmpl| {
                        if has_user_css {
                            tmpl << html !(link(rel="stylesheet", href="user_css.css"));
                        }
                    }
                    link(rel="stylesheet", href="highlight.css");
                    link(rel="stylesheet", href="fontawesome/css/fontawesome.css");
                    script(src="highlight.js");
                    script(src="unveil.js");
                    |tmpl| {
                        if self.live_reload {
                            tmpl << html !(script(src="livereload.js"));
                        }
                    }
                }
                body {
                   div(onclick="next_slide_right()", class="arrow-right") {
                       i(class="fas fa-chevron-right");
                   }

                   div(onclick="next_slide_left()", class="arrow-left") {
                        i(class="fas fa-chevron-left");
                   }
                   : Raw(&self.html)
                }
            }
        };

        Ok((css, format!("{}", html)))
    }
    fn split_slylematters(slide_content: &str) -> (Option<String>, String) {
        // No stylematters : return the content as it is
        if !PAGE_RE.is_match(slide_content) {
            return (None, slide_content.to_owned());
        }

        // 2. extract the style matter and the content
        let caps = PAGE_RE.captures(slide_content).unwrap();
        // caps[0] is the full match
        // caps[1] => style matter
        // caps[2] => content
        (Some(caps[1].to_string()), caps[2].to_string())
    }

    fn insert_user_class(&mut self) {
        let user_class_start = r#"[class=&quot;"#;
        let user_class_end = "&quot;]";

        let mut result = String::new();
        let mut last_end = 0;

        for (start, part) in self.html.match_indices(user_class_start) {
            let end_idx = self.html[start..self.html.len()].find(user_class_end);
            result.push_str(&self.html[last_end..start]);

            // We found a user defined class
            if let Some(end) = end_idx {
                let class_name = &self.html[start + user_class_start.len()..start + end];
                let class_attr = format!(" class=\"{}\"", class_name);
                // go to parent tag in the current html
                if self.html[0..start].rfind('>').is_some() {
                    // corresponding tag in the result string
                    if let Some(open_tag_closing_in_result) = result.rfind('>') {
                        let tag_to_insert = format!(" {}", class_attr);
                        result.insert_str(open_tag_closing_in_result, &tag_to_insert);
                    }

                    // next start after the markdown extension class
                    last_end = start + part.len() + class_name.len() + user_class_end.len();
                }
            } else {
                eprintln!("Unmatched class attribute");
            }
        }

        if last_end < self.html.len() {
            result.push_str(&self.html[last_end..self.html.len()]);
        }
        self.html = result;
    }


    // This is just like String::replace implementation with index
    fn insert_playpen_button(&mut self) {
        let code_tag = r#"<code class="language-rust">"#;
        let mut result = String::new();
        let mut last_end = 0;
        let mut count = 0;

        for (start, part) in self.html.match_indices(code_tag) {
            let code_block_id = format!("rust-code-block-{}", count);
            let button = html! {
                div(class="btn-code") {
                    i(class="fas fa-copy btn-copy", onclick="clipboard(this.id)", id=&code_block_id);
                    i(class="fas fa-play btn-playpen", onclick="play_playpen(this.id)", id=&code_block_id);
                }
            };
            let insert = format!("{}{}", button.to_string(), code_tag);

            result.push_str(unsafe { self.html.get_unchecked(last_end..start) });
            result.push_str(&insert);
            last_end = start + part.len();
            count += 1;
        }
        result.push_str(unsafe { self.html.get_unchecked(last_end..self.html.len()) });
        self.html = result;
    }

    fn markdown_to_html(&mut self) {
        let mut html_ouput = String::new();
        let mut scss_output = String::new();
        self.markdown
            .iter()
            .map(|content| Preprocessor::split_slylematters(content))
            .enumerate()
            .map(|(idx, (stylematter, markdown))| {
                let parser = Parser::new_ext(&markdown, Options::empty());
                let mut html = String::new();
                html::push_html(&mut html, parser);
                (idx, html, stylematter)
            })
            .for_each(|(idx, html, stylematter)| {
                let idx = &format!("unveil-slide-{}", idx);

                if let Some(stylematter) = stylematter {
                    let scss_block = &format!("#{} {{ {} }}", idx, stylematter);
                    scss_output.push_str(scss_block);
                }

                html_ouput.push_str(&format!(
                    "{}",
                    html! {
                        section(id=idx) { article { : Raw(&html) } }
                    }
                ));
            });

        self.scss = scss_output;
        self.html = html_ouput;
    }

    pub fn new(
        markdown: Vec<String>,
        live_reload: bool,
    ) -> Self {
        Preprocessor {
            markdown,
            live_reload,
            html: String::new(),
            scss: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::html::Preprocessor;

    #[test]
    fn should_replace_custom_classes() {
        let mut preprocessor = Preprocessor::new(vec![r#"[class="dummy"] Hello "#.into()], true);

        let output = preprocessor.build().unwrap();

        assert!(output.1.contains(r#"<p  class="dummy"> Hello </p>"#));
    }

    #[test]
    fn should_replace_do_nothing_if_no_custom_classes() {
        let mut preprocessor = Preprocessor::new(vec!["Hello".into()], true);

        let output = preprocessor.build().unwrap();

        assert!(output.1.contains("<p>Hello</p>"));
    }
}
