use crate::html::preprocessor::Preprocessor;
use anyhow::Result;
use horrorshow::{helper::doctype, prelude::*};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use sass_rs::Options as SassOption;

mod preprocessor;

// from zola https://github.com/getzola/zola/blob/1972e58823417a58eb1cc646ee346e7c3b04addb/components/front_matter/src/lib.rs
lazy_static! {
    static ref PAGE_RE: Regex =
        Regex::new(r"^[[:space:]]*\+\+\+\r?\n((?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$").unwrap();
}

pub struct HtmlBuilder {
    pub html: String,
    pub scss: String,
    pub markdown: Vec<String>,
    live_reload: bool,
}

impl HtmlBuilder {
    pub fn build(&mut self) -> Result<(Option<String>, String)> {
        self.markdown_to_html();
        self.html = Preprocessor::insert_user_class(&self.html);
        self.html = Preprocessor::insert_playpen_button(&self.html);

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
                }
                body {
                   div(onclick="next_slide_right()", class="arrow-right bounce-in") {
                       i(class="fas fa-chevron-right");
                   }

                   div(onclick="next_slide_left()", class="arrow-left bounce-in") {
                        i(class="fas fa-chevron-left");
                   }
                   : Raw(&self.html);
                   script(src="highlight.js");
                   script(src="clipboard.js");
                   script(src="unveil.js");
                   |tmpl| {
                       if self.live_reload {
                         tmpl << html !(script(src="livereload.js"));
                       }
                   }
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

    fn markdown_to_html(&mut self) {
        let mut html_ouput = String::new();
        let mut scss_output = String::new();
        self.markdown
            .iter()
            .map(|content| HtmlBuilder::split_slylematters(content))
            .enumerate()
            .map(|(idx, (stylematter, markdown))| {
                let parser = Parser::new_ext(&markdown, Options::empty());
                let mut html = String::new();
                html::push_html(&mut html, parser);
                (idx, html, stylematter)
            })
            .for_each(|(idx, html, stylematter)| {
                let idx = &format!("unveil-slide-{}", idx);

                // If there is a style matter block wrap the inner scss in the section id block
                if let Some(stylematter) = stylematter {
                    let scss_block = &format!("#{} {{ {} }}", idx, stylematter);
                    scss_output.push_str(scss_block);
                }

                // push our slide sections to the body
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
        HtmlBuilder {
            markdown,
            live_reload,
            html: String::new(),
            scss: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::html::HtmlBuilder;

    #[test]
    fn should_replace_custom_classes() {
        let mut preprocessor = HtmlBuilder::new(vec![r#"[class="dummy"] Hello "#.into()], true);

        let output = preprocessor.build().unwrap();

        assert!(output.1.contains(r#"<p class="dummy"> Hello </p>"#));
    }

    #[test]
    fn should_replace_do_nothing_if_no_custom_classes() {
        let mut preprocessor = HtmlBuilder::new(vec!["Hello".into()], true);

        let output = preprocessor.build().unwrap();

        assert!(output.1.contains("<p>Hello</p>"));
    }
}
