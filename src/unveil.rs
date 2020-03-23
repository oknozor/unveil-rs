use anyhow::Result;
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::{
    assets::{CSS, HIGHLIGHT_CSS, HIGHLIGHT_JS, JS, LANDING, LIVERELOAD_JS},
    config::UnveilConfig,
    helper,
};

use crate::{
    assets::{
        CLIPBOARD_JS,
        FONT_AWESOME,
        FONT_AWESOME_EOT,
        FONT_AWESOME_EOT_900,
        FONT_AWESOME_EOT_BRANDS,
        FONT_AWESOME_SVG,
        FONT_AWESOME_SVG_900,
        FONT_AWESOME_SVG_BRANDS,
        FONT_AWESOME_TTF,
        FONT_AWESOME_TTF_900,
        FONT_AWESOME_TTF_BRANDS,
        FONT_AWESOME_WOFF,
        FONT_AWESOME_WOFF2,
        FONT_AWESOME_WOFF2_900,
        FONT_AWESOME_WOFF2_BRANDS,
        FONT_AWESOME_WOFF_900,
        FONT_AWESOME_WOFF_BRANDS,
    },
    html::HtmlBuilder,
    server::Server,
};
use std::path::PathBuf;

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
    fn get_markdown_from_file() -> Result<Vec<String>> {
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
        Ok(markdown_contents)
    }

    /// Build a assets file from the markdown content located in `slides/`
    pub fn build(&mut self, server: &Server) -> Result<()> {
        // Generate html from markdown files in
        let markdowns = UnveilProject::get_markdown_from_file()?;
        let mut processor = HtmlBuilder::new(markdowns, self.livereload);

        let (user_css, html) = processor.build()?;
        let public = PathBuf::from("public");

        // Double check we are actually in an unveil project
        let config = UnveilConfig::from_disk("unveil.toml")?;

        // User has remove gitignore and we now need to recreate it
        if config.gitignore {
            helper::fs::write_file(".gitignore", b"public")?;
        }

        if !public.exists() {
            std::fs::create_dir("public")?;
        }

        helper::fs::replace("public/index.html", html.as_bytes())?;
        helper::fs::write_file("public/unveil.js", JS)?;

        if let Some(css) = user_css {
            helper::fs::replace("public/user_css.css", css.as_bytes())?;
        }

        helper::fs::write_file("public/highlight.css", HIGHLIGHT_CSS)?;
        helper::fs::write_file("public/livereload.js", LIVERELOAD_JS)?;

        // Replace livereload.js in case changes were made to the ws host and port
        let livereload = format!(r#"let socket = new WebSocket("ws://{}:{}");{}"#,
            server.hostname,
            server.ws_port,
            String::from_utf8(LIVERELOAD_JS.to_vec()).unwrap()
        );
        helper::fs::replace("public/livereload.js", livereload.as_bytes())?;

        helper::fs::write_file("public/clipboard.js", CLIPBOARD_JS)?;
        helper::fs::write_file("public/highlight.js", HIGHLIGHT_JS)?;

        helper::fs::create_dir("public/fontawesome");
        helper::fs::create_dir("public/fontawesome/webfonts");
        helper::fs::create_dir("public/fontawesome/css");

        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-regular-400.eot",
            FONT_AWESOME_EOT,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-regular-400.svg",
            FONT_AWESOME_SVG,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-regular-400.ttf",
            FONT_AWESOME_TTF,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-regular-400.woff",
            FONT_AWESOME_WOFF,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-regular-400.woff2",
            FONT_AWESOME_WOFF2,
        )?;

        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-brands-400.eot",
            FONT_AWESOME_EOT_BRANDS,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-brands-400.svg",
            FONT_AWESOME_SVG_BRANDS,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-brands-400.ttf",
            FONT_AWESOME_TTF_BRANDS,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-brands-400.woff",
            FONT_AWESOME_WOFF_BRANDS,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-brands-400.woff2",
            FONT_AWESOME_WOFF2_BRANDS,
        )?;

        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-solid-900.eot",
            FONT_AWESOME_EOT_900,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-solid-900.svg",
            FONT_AWESOME_SVG_900,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-solid-900.ttf",
            FONT_AWESOME_TTF_900,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-solid-900.woff",
            FONT_AWESOME_WOFF_900,
        )?;
        helper::fs::write_file(
            "public/fontawesome/webfonts/fa-solid-900.woff2",
            FONT_AWESOME_WOFF2_900,
        )?;
        helper::fs::write_file("public/fontawesome/css/fontawesome.css", FONT_AWESOME)?;

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

        // Add default gitignore
        let mut gitignore = File::create(&format!("{}/.gitignore", project_name))?;
        gitignore.write_all(b"pubic")?;

        // Add a default example slides
        let mut landing = File::create(&format!("{}/slides/landing.md", project_name))?;
        landing.write_all(LANDING)?;

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

        let path = PathBuf::from("slides").join(&filename);

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
        hostname: Option<&str>,
        http_port: Option<i32>,
        ws_port: Option<i32>,
    ) -> Result<()> {
        let server = Server::default()
            .with_hostname(hostname)
            .with_http_port(http_port)
            .with_ws_port(ws_port);

        self.build(&server)?;

        server.serve()
    }
}
