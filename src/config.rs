use std::path::PathBuf;
use anyhow::Result;
use std::fs;

// Do we actually need this ?
// static file are duplicated (in memory + in config)
// Having the default config in XDG_CONFIG allow user modification of the default theme
// In memory static content allow to
//      - reset to the default config
//      - init the default config without a http call
pub struct UnveilCommon {
    js_contents: String,
    css_contents: String,
}

impl UnveilCommon {
    pub fn default() -> Result<Self> {
        let mut unveil_css_path: PathBuf = dirs::config_dir()
            .unwrap_or_else(|| panic!("Could not resolve config dir"));

        unveil_css_path.push("unveil");
        unveil_css_path.push("unveil.css");

        let mut unveil_js_path: PathBuf = dirs::config_dir()
            .unwrap_or_else(|| panic!("Could not resolve config dir"));

        unveil_js_path.push("unveil");
        unveil_js_path.push("unveil.js");

        let css_contents = fs::read_to_string(unveil_css_path)?;
        let js_contents = fs::read_to_string(unveil_js_path)?;

        Ok(UnveilCommon { js_contents, css_contents })
    }
}