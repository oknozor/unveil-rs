use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use pulldown_cmark::{Parser, html, Options};

use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use crate::config::UnveilConfig;
use crate::generated::{CSS, JS, LANDING, SLIDE_EXAMPLE, LIVERELOAD_JS};
use std::path::{Path, PathBuf};
use iron::headers;
use iron::{Iron, Chain, AfterMiddleware, Request, IronError, IronResult, Response, status, Set};
use std::ffi::OsStr;
use crate::watcher;


pub struct UnveilProject {
    pub root: PathBuf,
    pub markdown: Vec<String>,
    pub livereload: bool,
}

impl Default for UnveilProject {
    fn default() -> Self {
        UnveilProject {
            root: PathBuf::from("."),
            markdown: vec![],
            livereload: true
        }
    }
}

impl UnveilProject {
    /// get markdowns slides as strings
    fn get_dir_files(&mut self) -> Result<()> {
        let mut markdown_contents = vec![];
        let config = UnveilConfig::from_disk("unveil.toml")?;

        // Read slide names from config and lookup the corresponding slide in the
        // slides directory, this allow to order slides rendering
        for slide_name in config.slides.iter() {
            let path = PathBuf::from(&format!("slides/{}", slide_name));
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            markdown_contents.push(contents);
        }

        self.markdown = markdown_contents;
        Ok(())
    }

    /// Build the `index.hml` html page
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
                    |tmpl| {
                        if self.livereload {
                            tmpl << html !(script(src="livereload.js"));
                        }
                    }
                }
                body(onscroll="scroll_changed()") {
                   button(onclick="next_slide_left()", class="arrow-left") { : "<"}
                   button(onclick="next_slide_right()", class="arrow-right") { : ">"}
                   : Raw(&sections)
                }
            }
        };

        format!("{}", html)
    }

    /// Convert each markdown pages to html
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
                    section(id=idx) { article { : Raw(&html) } }
                }));
            });

        sections
    }


    /// Build a static file from the markdown content located in `slides/`
    pub fn build(&mut self) -> Result<()> {

        // Generate html from markdown files in
        self.get_dir_files()?;
        let html = self.to_html();

        // Generate static site
        let public = Path::new("public");
        // Double check we are actually in an unveil project
        let config = Path::new("unveil.toml");
        if public.exists() && config.exists() {
            std::fs::remove_dir_all(public)?;
        }

        std::fs::create_dir("public")?;

        let mut index = File::create("public/index.html")?;
        index.write_all(html.as_bytes())?;

        let mut css = File::create("public/unveil.css")?;
        css.write_all(CSS.as_bytes())?;

        let mut js = File::create("public/unveil.js")?;
        js.write_all(JS.as_bytes())?;

        let mut livereload = File::create("public/livereload.js")?;
        livereload.write_all(LIVERELOAD_JS.as_bytes())?;

        Ok(())
    }

    /// Initialize a template project
    pub fn init(&mut self, project_name: Option<&str>) -> Result<()> {
        let project_name = if let Some(project_name) = project_name {
            project_name
        } else {
            "unveil"
        };

        // Create slides dir
        std::fs::create_dir(project_name)?;
        std::fs::create_dir(&format!("{}/slides", project_name))?;

        // Add a default example slides
        let mut landing = File::create(&format!("{}/slides/landing.md", project_name))?;
        landing.write_all(LANDING.as_bytes())?;
        let mut landing = File::create(&format!("{}/slides/slide.md", project_name))?;
        landing.write_all(SLIDE_EXAMPLE.as_bytes())?;

        // Generate default config
        let mut config_file = File::create(&format!("{}/unveil.toml", project_name))?;
        let default_config = toml::to_string(&UnveilConfig::default())?;
        config_file.write_all(default_config.as_bytes())?;

        Ok(())
    }

    pub fn new_slide(&mut self, _: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn serve(&mut self, port: Option<i32>) -> Result<()> {
        let address = match port {
            Some(port) => format!("localhost:{}", port),
            None => "localhost:7878".into()
        };

        let mut chain = Chain::new(staticfile::Static::new(PathBuf::from("public/")));

        chain.link_after(NoCache);
        chain.link_after(ErrorRecover);
        let _iron = Iron::new(chain)
            .http(&*address)?;

        let ws_server =
            ws::WebSocket::new(|_| |_| Ok(()))?;

        let broadcaster = ws_server.broadcaster();
        std::thread::spawn(move || {
            ws_server.listen("127.0.0.1:3000").expect("Error Opening websocket");
        });

        let serving_url = format!("http://{}", address);
        println!("Serving on: {}", serving_url);

        open(serving_url);
        let mut book_dir = std::env::current_dir()?;
        book_dir.push("slides");

        let mut paths = vec![];

        let entries = fs::read_dir(book_dir)?;

        for entry in entries {
            let entry = entry?;
            paths.push(entry.path());
        };

        watcher::trigger_on_change(|paths| {
            println!("Files changed: {:?}", paths);
            println!("Building presentation...");

            let mut project = UnveilProject::default();
            let result = project.build();

            if let Err(e) = result {
                eprintln!("Unable to load the presentation : {}", e);
            } else {
                let _ = broadcaster.send("reload");
            }
        });

        Ok(())
    }
}

struct ErrorRecover;

struct NoCache;

impl AfterMiddleware for NoCache {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::CacheControl(vec![
            headers::CacheDirective::NoStore,
            headers::CacheDirective::MaxAge(0u32),
        ]));

        Ok(res)
    }
}

impl AfterMiddleware for ErrorRecover {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        match err.response.status {
            // each error will result in 404 response
            Some(_) => Ok(err.response.set(status::NotFound)),
            _ => Err(err),
        }
    }
}

fn open<P: AsRef<OsStr>>(path: P) {
    if let Err(e) = open::that(path) {
        eprintln!("Error opening web browser: {}", e);
    }
}