use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use pulldown_cmark::{Parser, html, Options};

pub struct UnveilProject {
    pub markdown: Vec<String>
}

impl UnveilProject {
    fn get_dir_files(&mut self) -> Result<()> {
        let mut markdown_contents = vec![];

        for entry in fs::read_dir(".")? {
            let dir = entry?;
            if dir.file_type()?.is_file() {
                let mut file = File::open(dir.path())?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                markdown_contents.push(contents);
            }
        }
        self.markdown = markdown_contents;
        Ok(())
    }

    fn to_html(&self) -> String {
        let mut res = String::new();

        self.markdown.iter()
            .for_each(|str| res.push_str(str));

        let mut options = Options::empty();
        let parser = Parser::new_ext(&res, options);
        let mut buffer = String::new();
        html::push_html(&mut buffer, parser);
        buffer
    }

    pub fn build(&mut self) -> Result<()> {
        self.get_dir_files()?;
        let html = self.to_html();
        std::fs::create_dir("public")?;
        let mut file = File::create("public/index.html")?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }
}