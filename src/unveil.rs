use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::{
    config::UnveilConfig,
    assets::{CSS, JS, LANDING, LIVERELOAD_JS, SLIDE_EXAMPLE, HIGHLIGHT_CSS, HIGHLIGHT_JS},
    watcher,
};
use horrorshow::{helper::doctype, prelude::*};
use iron::{
    headers,
    status,
    AfterMiddleware,
    Chain,
    Iron,
    IronError,
    IronResult,
    Request,
    Response,
    Set,
};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub struct UnveilProject {
    pub root: PathBuf,
    pub markdown: Vec<String>,
    pub livereload: bool,
}

impl Default for UnveilProject {
    fn default() -> Self {
        UnveilProject {
            // TODO: handle this correctly and use it!
            root: PathBuf::from("."),
            markdown: vec![],
            livereload: true,
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
                    link(rel="stylesheet", href="highlight.css");
                    script(src="unveil.js");
                    script(src="highlight.js");
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
                sections.push_str(&format!(
                    "{}",
                    html! {
                        section(id=idx) { article { : Raw(&html) } }
                    }
                ));
            });

        sections
    }

    /// Build a assets file from the markdown content located in `slides/`
    pub fn build(&mut self) -> Result<()> {
        // Generate html from markdown files in
        self.get_dir_files()?;
        let html = self.to_html();

        let public = PathBuf::from("public");

        // Double check we are actually in an unveil project
        let config = Path::new("unveil.toml");
        if !public.exists() {
            std::fs::create_dir("public")?;
        }

        if config.exists() {
            // TODO : we actually need to separate the build/init/serve/clean logic
            // Generate assets site
            let index = PathBuf::from("public/index.html");
            let js = PathBuf::from("public/livereload.js");
            let live_reload = PathBuf::from("public/unveil.js");
            let highlight_js = PathBuf::from("public/highlight.js");
            let highlight_css = PathBuf::from("public/highlight.css");

            if index.exists() {
                std::fs::remove_file(index)?;
            }

            if js.exists() {
                std::fs::remove_file(js)?;
            }

            if live_reload.exists() {
                std::fs::remove_file(live_reload)?;
            }

            if highlight_js.exists() {
                std::fs::remove_file(highlight_js)?;
            }

            if highlight_css.exists() {
                std::fs::remove_file(highlight_css)?;
            }


            let mut index = File::create("public/index.html")?;
            index.write_all(html.as_bytes())?;

            let mut js = File::create("public/unveil.js")?;
            js.write_all(JS)?;

            let mut livereload = File::create("public/livereload.js")?;
            livereload.write_all(LIVERELOAD_JS)?;

            let mut live_reload = File::create("public/highlight.js")?;
            live_reload.write_all(LIVERELOAD_JS)?;

            let mut js = File::create("public/highlight.js")?;
            js.write_all(HIGHLIGHT_JS)?;

            let mut js = File::create("public/highlight.css")?;
            js.write_all(HIGHLIGHT_CSS)?;
        }

        // We don't overwrite CSS by default
        let css = PathBuf::from("public/unveil.css");
        if !css.exists() {
            let mut css = File::create("public/unveil.css")?;
            css.write_all(CSS)?;
        }

        Ok(())
    }

    /// Initialize a template project
    pub fn init(
        &mut self,
        project_name: Option<&str>,
    ) -> Result<()> {
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
        landing.write_all(LANDING)?;
        let mut landing = File::create(&format!("{}/slides/slide.md", project_name))?;
        landing.write_all(SLIDE_EXAMPLE)?;

        // Generate default config
        let mut config_file = File::create(&format!("{}/unveil.toml", project_name))?;
        let default_config = toml::to_string(&UnveilConfig::default())?;
        config_file.write_all(default_config.as_bytes())?;

        Ok(())
    }

    pub fn new_slide(
        &mut self,
        name: &str,
    ) -> Result<()> {
        let filename = if name.ends_with(".md") {
            name.into()
        } else {
            format!("{}.md", name)
        };

        let mut path = PathBuf::from("slides");
        path.push(&filename);

        let mut config = UnveilConfig::from_disk("unveil.toml")?;

        File::create(path).map(|_| ())?;
        config.slides.push(filename);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("unveil.toml")?;

        file.write_all(toml::to_string(&config)?.as_bytes())
            .map_err(|err| anyhow!("Error writing to unveil.toml : {}", err))
    }

    pub fn serve(
        &mut self,
        port: Option<i32>,
    ) -> Result<()> {
        let address = match port {
            Some(port) => format!("localhost:{}", port),
            None => "localhost:7878".into(),
        };

        let mut chain = Chain::new(staticfile::Static::new(PathBuf::from("public/")));

        chain.link_after(NoCache);
        chain.link_after(ErrorRecover);
        let _iron = Iron::new(chain).http(&*address)?;

        let ws_server = ws::WebSocket::new(|_| |_| Ok(()))?;

        let broadcaster = ws_server.broadcaster();
        std::thread::spawn(move || {
            ws_server
                .listen("127.0.0.1:3000")
                .expect("Error Opening websocket");
        });

        let serving_url = format!("http://{}", address);
        println!("Serving on: {}", serving_url);

        let mut slides_dir = std::env::current_dir()?;
        slides_dir.push("slides");

        let mut paths = vec![];

        let entries = fs::read_dir(slides_dir)?;

        for entry in entries {
            let entry = entry?;
            paths.push(entry.path());
        }

        paths.push(PathBuf::from("unveil.toml"));
        paths.push(PathBuf::from("public/unveil.css"));

        open(serving_url);

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
    fn after(
        &self,
        _: &mut Request,
        mut res: Response,
    ) -> IronResult<Response> {
        res.headers.set(headers::CacheControl(vec![
            headers::CacheDirective::NoStore,
            headers::CacheDirective::MaxAge(0u32),
        ]));

        Ok(res)
    }
}

impl AfterMiddleware for ErrorRecover {
    fn catch(
        &self,
        _: &mut Request,
        err: IronError,
    ) -> IronResult<Response> {
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
