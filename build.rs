use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let js_content = fs::read_to_string("template/unveil.js").unwrap();
    let css_content = fs::read_to_string("template/unveil.css").unwrap();

    let js_const = format!("pub const JS: &str = r#\"{}\"#;", js_content);
    let css_const = format!("pub const CSS: &str = r#\"{}\"#;", css_content);

    let mut const_rile = File::create("src/generated.rs").unwrap();
    const_rile.write_all(js_const.as_bytes()).unwrap();
    const_rile.write_all(css_const.as_bytes()).unwrap();
}