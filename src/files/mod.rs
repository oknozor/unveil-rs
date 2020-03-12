use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use pulldown_cmark::{Parser, html, Options};


use horrorshow::prelude::*;
use horrorshow::helper::doctype;


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
        let mut sections = String::new();

        let inner = self.markdown.iter()
            .map(|str| {
                let mut options = Options::empty();
                let parser = Parser::new_ext(&str, options);
                let mut buffer = String::new();
                html::push_html(&mut buffer, parser);

                buffer
            }).for_each(|html| {
            let section = format!("{}",
                                  html! {
                        section {
                            : Raw(&html)
                        }
                    });
            sections.push_str(&section);
        });

        let html = html! {
            : doctype::HTML;
            html {
                head {
                    Raw("<meta charset=\"utf-8\" />")
                    title : "Unveil";
                }
                body {
                   : Raw(&sections)
                }
            }
        };

        format!("{}", html)
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