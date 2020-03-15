use anyhow::{Error, Result};
use std::{fs::File, io::Read, path::Path, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct UnveilConfig {
    name: String,
    language: String,
    pub slides: Vec<String>,
    pub user_theme: Option<String>,
    pub gitignore: bool,
}

impl Default for UnveilConfig {
    fn default() -> Self {
        UnveilConfig {
            name: "unveil".to_string(),
            language: "EN".to_string(),
            slides: vec!["landing.md".into()],
            user_theme: None,
            gitignore: true,
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
}
