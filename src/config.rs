use crate::assets::{CSS_DARK_THEME, CSS_THEME};
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct UnveilConfig {
    name: String,
    language: String,
    pub slides: Vec<String>,
    pub gitignore: bool,
    pub theme: String,
}

impl Default for UnveilConfig {
    fn default() -> Self {
        UnveilConfig {
            name: "unveil".to_string(),
            language: "EN".to_string(),
            slides: vec!["landing.md".into()],
            gitignore: true,
            theme: "default".to_string(),
        }
    }
}

impl FromStr for UnveilConfig {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self> {
        let toml = toml::from_str(src).expect("Error loading config");
        Ok(toml)
    }
}

impl UnveilConfig {
    pub fn from_disk<P: AsRef<Path>>(config_file: P) -> Result<UnveilConfig> {
        let mut buffer = String::new();
        File::open(config_file)?.read_to_string(&mut buffer)?;

        UnveilConfig::from_str(&buffer)
    }

    pub fn get_theme(&self) -> Result<Vec<u8>> {
        match self.theme.as_str() {
            "default" => Ok(CSS_THEME.to_vec()),
            "dark" => Ok(CSS_DARK_THEME.to_vec()),
            custom_theme => {
                let content = std::fs::read_to_string(format!("public/{}", custom_theme))?;
                let bytes = content.as_bytes().to_owned();
                Ok(bytes)
            }
        }
    }
}
