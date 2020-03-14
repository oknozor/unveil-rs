use anyhow::Result;
use std::{fs::File, io::Write, path::PathBuf};

pub fn replace(
    filename: &str,
    content: &[u8],
) -> Result<()> {
    let file = PathBuf::from(filename);

    if file.exists() {
        std::fs::remove_file(file)?;
    }

    let mut file = File::create(filename)?;
    file.write_all(content)
        .map_err(|err| anyhow!("Could not write to file {} : {}", filename, err))
}

#[cfg(test)]
pub mod test {
    use crate::helper::files::replace;
    use tempfile::NamedTempFile;

    #[test]
    fn should_replace_existing_file() {
        let file = NamedTempFile::new().unwrap();
        let filename = file.path().to_str().unwrap();

        replace(filename, "Dummy text".as_bytes()).unwrap();

        assert!(file.path().exists());
        assert_eq!(std::fs::read_to_string(file.path()).unwrap(), "Dummy text");
    }
}
