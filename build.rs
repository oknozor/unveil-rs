use std::{fs, fs::File, io::Write};

// default static files are generated from in memory constants,
// this might not be the best solution but it will do for now
fn main() {
    let js_content = fs::read_to_string("template/unveil.js").unwrap();
    let css_content = fs::read_to_string("template/unveil.css").unwrap();
    let landing_slide = fs::read_to_string("template/landing.md").unwrap();
    let example_slide = fs::read_to_string("template/slide.md").unwrap();
    let livereload = fs::read_to_string("template/livereload.js").unwrap();

    let js_const = format!("pub const JS: &str = r#\"{}\"#;", js_content);
    let css_const = format!("pub const CSS: &str = r#\"{}\"#;", css_content);
    let landing_slide_const = format!("pub const LANDING: &str = r#\"{}\"#;", landing_slide);
    let example_slide_const = format!("pub const SLIDE_EXAMPLE: &str = r#\"{}\"#;", example_slide);
    let livereload_const = format!("pub const LIVERELOAD_JS: &str = r#\"{}\"#;", livereload);

    let mut const_rile = File::create("src/generated.rs").unwrap();
    const_rile.write_all(js_const.as_bytes()).unwrap();
    const_rile.write_all(css_const.as_bytes()).unwrap();
    const_rile
        .write_all(landing_slide_const.as_bytes())
        .unwrap();
    const_rile
        .write_all(example_slide_const.as_bytes())
        .unwrap();
    const_rile.write_all(livereload_const.as_bytes()).unwrap();
}
