pub(crate) struct Preprocessor;

const RUST_CODE_TAG: &str = r#"<code class="language-rust">"#;
const MD_CLASS_START: &str = r#"[class=&quot;"#;
const MD_CLASS_END: &str = "&quot;]";

impl Preprocessor {
    pub fn insert_playpen_button(html: &str) -> String {
        let mut result = String::new();
        let mut last_end = 0;
        let mut count = 0;

        for (start, part) in html.match_indices(RUST_CODE_TAG) {
            let code_block_id = format!("rust-code-block-{}", count);
            let button = html! {
                div(class="btn-code-container") {
                    div(class="btn-code") {
                        i(class="fas fa-copy bounce-in btn-copy", id=&code_block_id);
                        i(class="fas fa-play btn-playpen", onclick="play_playpen(this.id)", id=&code_block_id);
                    }
                }
            };
            let insert = format!("{}{}", button.to_string(), RUST_CODE_TAG);

            result.push_str(&html[last_end..start]);
            result.push_str(&insert);
            last_end = start + part.len();
            count += 1;
        }
        result.push_str(&html[last_end..html.len()]);
        result
    }

    pub fn insert_user_class(html: &str) -> String {
        let mut result = String::new();
        let mut last_end = 0;

        for (start, part) in html.match_indices(MD_CLASS_START) {
            let end_idx = html[start..html.len()].find(MD_CLASS_END);
            result.push_str(&html[last_end..start]);

            // We found a user defined class
            if let Some(end) = end_idx {
                let class_name = &html[start + MD_CLASS_START.len()..start + end];
                let class_attr = format!(" class=\"{}\"", class_name);
                // go to parent tag in the current html
                if html[0..start].rfind('>').is_some() {
                    // corresponding tag in the result string
                    if let Some(open_tag_closing_in_result) = result.rfind('>') {
                        result.insert_str(open_tag_closing_in_result, &class_attr);
                    }

                    // next start after the markdown extension class
                    last_end = start + part.len() + class_name.len() + MD_CLASS_END.len();
                }
            } else {
                eprintln!("Unmatched class attribute");
            }
        }

        if last_end < html.len() {
            result.push_str(&html[last_end..html.len()]);
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use crate::html::preprocessor::{Preprocessor, MD_CLASS_END, MD_CLASS_START, RUST_CODE_TAG};

    #[test]
    fn should_insert_playpen_buttons() {
        let input = format!("{}let a = 1;</code>", RUST_CODE_TAG);

        let output = Preprocessor::insert_playpen_button(&input);

        let expected = html! {
                div(class="btn-code") {
                    i(class="fas fa-copy bounce-in btn-copy", id="rust-code-block-0");
                    i(class="fas fa-play btn-playpen", onclick="play_playpen(this.id)", id="rust-code-block-0");
                }
                code(class="language-rust") {
                    : "let a = 1;";
                }
            }.to_string();

        assert_eq!(output, expected);
    }

    #[test]
    fn should_insert_playpen_buttons_on_multiple_block() {
        let input = format!(
            "{}let a = 1;</code><p>Foo</p>{}let a = 1;</code>",
            RUST_CODE_TAG, RUST_CODE_TAG
        );

        let output = Preprocessor::insert_playpen_button(&input);

        let expected = html! {
                div(class="btn-code") {
                    i(class="fas fa-copy bounce-in btn-copy", id="rust-code-block-0");
                    i(class="fas fa-play btn-playpen", onclick="play_playpen(this.id)", id="rust-code-block-0");
                }
                code(class="language-rust") {
                    : "let a = 1;";
                }
                p { : "Foo"}
                div(class="btn-code") {
                    i(class="fas fa-copy bounce-in btn-copy", id="rust-code-block-1");
                    i(class="fas fa-play btn-playpen", onclick="play_playpen(this.id)", id="rust-code-block-1");
                }
                code(class="language-rust") {
                    : "let a = 1;";
                }
            }.to_string();

        assert_eq!(output, expected);
    }

    #[test]
    fn should_replace_markdown_classes_right_with_html_ones() {
        let input = format!(
            "<p>Html content{}super-class{}</p>",
            MD_CLASS_START, MD_CLASS_END
        );

        let output = Preprocessor::insert_user_class(&input);

        let expected = html! {
            p(class="super-class") { : "Html content"; }
        }
        .to_string();

        assert_eq!(output, expected);
    }

    #[test]
    fn should_replace_markdown_classes_left_with_html_ones() {
        let input = format!(
            "<p>{}super-class{}Html content</p>",
            MD_CLASS_START, MD_CLASS_END
        );

        let output = Preprocessor::insert_user_class(&input);

        let expected = html! {
            p(class="super-class") { : "Html content"; }
        }
        .to_string();

        assert_eq!(output, expected);
    }
}
