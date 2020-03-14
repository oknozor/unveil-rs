use horrorshow::{helper::doctype, prelude::*};
use pulldown_cmark::{html, Options, Parser};

pub struct Preprocessor {
    pub html: String,
    pub markdown: Vec<String>,
    live_reload: bool,
}

impl Preprocessor {
    pub fn build(&mut self) -> String {
        self.markdown_to_html();
        self.insert_playpen_button();

        let html = html! {
            : doctype::HTML;
            html(lang="EN") {
                head {
                    meta(charset="utf8");
                    title : "Unveil";
                    link(rel="stylesheet", href="unveil.css");
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

        format!("{}", html)
    }

    fn insert_playpen_button(&mut self) {
        let code_tag = r#"<code class="language-rust">"#;
        let mut result = String::new();
        let mut last_end = 0;
        let mut count = 0;

        for (start, part) in self.html.match_indices(code_tag) {
            let code_block_id = format!("rust-code-block-{}", count);
            let button = html! {
                div(class="btn-code") {
                    i(class="fas fa-copy btn-copy", onclick="clipboard()");
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
        let mut out = String::new();
        self.markdown
            .iter()
            .enumerate()
            .map(|(idx, str)| {
                let parser = Parser::new_ext(&str, Options::empty());
                let mut buffer = String::new();
                html::push_html(&mut buffer, parser);

                (idx, buffer)
            })
            .for_each(|(idx, html)| {
                let idx = &format!("unveil-slide-{}", idx);
                out.push_str(&format!(
                    "{}",
                    html! {
                        section(id=idx) { article { : Raw(&html) } }
                    }
                ));
            });

        self.html = out;
    }

    pub fn new(
        markdown: Vec<String>,
        live_reload: bool,
    ) -> Self {
        Preprocessor {
            markdown,
            live_reload,
            html: String::new(),
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
