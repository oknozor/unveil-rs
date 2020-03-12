use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use pulldown_cmark::{Parser, html, Options};


use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use crate::config::UnveilCommon;


pub struct UnveilProject {
    pub markdown: Vec<String>
}

impl UnveilProject {
    /// get the markdown pages as strings
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

    /// build the main html page
    fn to_html(&self) -> String {
        let sections = self.markdown_to_html_sections();

        let html = html! {
            : doctype::HTML;
            html(lang="EN") {
                head {
                    meta(charset="utf8");
                    title : "Unveil";
                    link(rel="stylesheet", href="unveil.css");
                    script(src="unveil.js");
                }
                body(onscroll="scroll_changed()") {
                   button(onclick="next_slide_left()", class="arrow-left");
                   button(onclick="next_slide_right()", class="arrow-right");
                   : Raw(&sections)
                }
            }
        };

        format!("{}", html)
    }

    /// convert each markdown pages to html
    fn markdown_to_html_sections(&self) -> String {
        let mut sections = String::new();
        self.markdown.iter().enumerate()
            .map(|(idx, str), | {
                let parser = Parser::new_ext(&str, Options::empty());
                let mut buffer = String::new();
                html::push_html(&mut buffer, parser);

                (idx, buffer)
            })
            .for_each(|(idx, html)| {
                let idx = &format!("unveil-slide-{}", idx);
                sections.push_str(&format!("{}", html! {
                    section(class=idx) { article { : Raw(&html) } }
                }));
            });

        sections
    }


    pub fn build(&mut self) -> Result<()> {
        self.get_dir_files()?;
        let common = UnveilCommon::default();

        let html = self.to_html();
        std::fs::create_dir("public")?;
        let mut file = File::create("public/index.html")?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }
}