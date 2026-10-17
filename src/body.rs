use anyhow::{Context, Result};
use std::{fs::OpenOptions, io::Read, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub enum BodySource {
    Stdin,
    File(PathBuf),
    Inline(String),
}

impl BodySource {
    pub fn parse(src: &str) -> Self {
        if src == "-" {
            BodySource::Stdin
        } else if let Some(src) = src.strip_prefix("@") {
            BodySource::File(PathBuf::from_str(src).unwrap())
        } else {
            BodySource::Inline(src.to_string())
        }
    }

    pub fn read(self) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        match self {
            BodySource::Stdin => {
                std::io::stdin().read_to_end(&mut data)?;
            }
            BodySource::File(path_buf) => {
                let mut file =
                    OpenOptions::new()
                        .read(true)
                        .open(&path_buf)
                        .with_context(|| {
                            format!("failed to open input file: {}", path_buf.display())
                        })?;
                file.read_to_end(&mut data)?;
            }
            BodySource::Inline(src) => data = src.into_bytes(),
        }
        Ok(data)
    }
}
