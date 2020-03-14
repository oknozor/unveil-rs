use anyhow::Result;
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::{
    assets::{CSS, HIGHLIGHT_CSS, HIGHLIGHT_JS, JS, LANDING, LIVERELOAD_JS, SLIDE_EXAMPLE},
    config::UnveilConfig,
    helper,
};

use crate::server::Server;
use std::path::{Path, PathBuf};

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

    /// Build a assets file from the markdown content located in `slides/`
    pub fn build(&mut self) -> Result<()> {
        // Generate html from markdown files in
        self.get_dir_files()?;
        let html = helper::html::build(&self.markdown, self.livereload);

        let public = PathBuf::from("public");

        // Double check we are actually in an unveil project
        let config = Path::new("unveil.toml");
        if !public.exists() {
            std::fs::create_dir("public")?;
        }

        if config.exists() {
            helper::files::replace("public/index.html", html.as_bytes())?;
            helper::files::replace("public/unveil.js", JS)?;
            helper::files::replace("public/livereload.js", LIVERELOAD_JS)?;
            helper::files::replace("public/highlight.js", LIVERELOAD_JS)?;
            helper::files::replace("public/highlight.js", HIGHLIGHT_JS)?;
            helper::files::replace("public/highlight.css", HIGHLIGHT_CSS)?;
        }

        // We don't overwrite CSS by default
        if !PathBuf::from("public/unveil.css").exists() {
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

    pub fn clean() -> Result<()> {
        fs::remove_dir_all("public")
            .map_err(|err| anyhow!("Unable to remove public directory : {}", err))
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
        &self,
        port: Option<i32>,
    ) -> Result<()> {
        let mut server = Server::default();

        if let Some(port) = port {
            server.with_port(port)
        }

        server.serve()
    }
}
