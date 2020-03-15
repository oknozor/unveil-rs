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

    // TODO : we probably need to get better assertions here
    #[test]
    fn should_append_playpen_button() {
        let markdown = vec![r#"
```rust
let a = 1;
```
"#
        .to_owned()];

        let mut preprocessor = Preprocessor::new(markdown, true);

        let html = preprocessor.build();

        assert_eq!(
            html,
            r#"<!DOCTYPE html><html lang="EN"><head><meta charset="utf8"><title>Unveil</title><link rel="stylesheet" href="unveil.css"><link rel="stylesheet" href="highlight.css"><script src="highlight.js"></script><script src="unveil.js"></script><script src="livereload.js"></script></head><body><div onclick="next_slide_right()" class="arrow-right"><svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="chevron-right" class="svg-inline--fa fa-chevron-right fa-w-10" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path fill="currentColor" d="M285.476 272.971L91.132 467.314c-9.373 9.373-24.569 9.373-33.941 0l-22.667-22.667c-9.357-9.357-9.375-24.522-.04-33.901L188.505 256 34.484 101.255c-9.335-9.379-9.317-24.544.04-33.901l22.667-22.667c9.373-9.373 24.569-9.373 33.941 0L285.475 239.03c9.373 9.372 9.373 24.568.001 33.941z"></path></svg></div><div onclick="next_slide_left()" class="arrow-left"><svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="chevron-left" class="svg-inline--fa fa-chevron-right fa-w-10" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path fill="currentColor" d="M34.52 239.03L228.87 44.69c9.37-9.37 24.57-9.37 33.94 0l22.67 22.67c9.36 9.36 9.37 24.52.04 33.9L131.49 256l154.02 154.75c9.34 9.38 9.32 24.54-.04 33.9l-22.67 22.67c-9.37 9.37-24.57 9.37-33.94 0L34.52 272.97c-9.37-9.37-9.37-24.57 0-33.94z"></path></svg></div><section id="unveil-slide-0"><article><pre><code class="language-rust"><div onclick="fetch_with_timeout()" class="btn-playpen"><svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="play" class="svg-inline--fa fa-play fa-w-14" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path fill="currentColor" d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"></path></svg></div>let a = 1;
</code></pre>
</article></section></body></html>"#
        )
    }

    #[test]
    fn should_append_playpen_buttons() {
        let markdown = vec![
            r#"
```rust
let a = 1;
```"#
                .to_owned(),
            r#"```rust
let b = 2;
```
        "#
            .to_owned(),
        ];

        let mut preprocessor = Preprocessor::new(markdown, true);

        let html = preprocessor.build();

        assert_eq!(
            html,
            r#"<!DOCTYPE html><html lang="EN"><head><meta charset="utf8"><title>Unveil</title><link rel="stylesheet" href="unveil.css"><link rel="stylesheet" href="highlight.css"><script src="highlight.js"></script><script src="unveil.js"></script><script src="livereload.js"></script></head><body><div onclick="next_slide_right()" class="arrow-right"><svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="chevron-right" class="svg-inline--fa fa-chevron-right fa-w-10" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path fill="currentColor" d="M285.476 272.971L91.132 467.314c-9.373 9.373-24.569 9.373-33.941 0l-22.667-22.667c-9.357-9.357-9.375-24.522-.04-33.901L188.505 256 34.484 101.255c-9.335-9.379-9.317-24.544.04-33.901l22.667-22.667c9.373-9.373 24.569-9.373 33.941 0L285.475 239.03c9.373 9.372 9.373 24.568.001 33.941z"></path></svg></div><div onclick="next_slide_left()" class="arrow-left"><svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="chevron-left" class="svg-inline--fa fa-chevron-right fa-w-10" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path fill="currentColor" d="M34.52 239.03L228.87 44.69c9.37-9.37 24.57-9.37 33.94 0l22.67 22.67c9.36 9.36 9.37 24.52.04 33.9L131.49 256l154.02 154.75c9.34 9.38 9.32 24.54-.04 33.9l-22.67 22.67c-9.37 9.37-24.57 9.37-33.94 0L34.52 272.97c-9.37-9.37-9.37-24.57 0-33.94z"></path></svg></div><section id="unveil-slide-0"><article><pre><code class="language-rust"><div onclick="fetch_with_timeout()" class="btn-playpen"><svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="play" class="svg-inline--fa fa-play fa-w-14" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path fill="currentColor" d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"></path></svg></div>let a = 1;
</code></pre>
</article></section><section id="unveil-slide-1"><article><pre><code class="language-rust"><div onclick="fetch_with_timeout()" class="btn-playpen"><svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="play" class="svg-inline--fa fa-play fa-w-14" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path fill="currentColor" d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"></path></svg></div>let b = 2;
</code></pre>
</article></section></body></html>"#
        )
    }
}
