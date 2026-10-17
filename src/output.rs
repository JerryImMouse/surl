use std::{
    fs::File,
    io::{BufWriter, IsTerminal},
    path::PathBuf,
};

use anyhow::{Context, Result};

pub struct Output {
    pub sink: Box<dyn std::io::Write>,
    pub colorize: bool,
}

impl Output {
    pub fn new(path: Option<PathBuf>, no_highlight: bool) -> Result<Self> {
        let colorize = path.is_none() && !no_highlight && std::io::stdout().is_terminal();
        let sink: Box<dyn std::io::Write> = match path {
            Some(path) => Box::new(BufWriter::new(File::create(&path).with_context(|| {
                format!("failed to open output file: `{}`", path.display())
            })?)),
            None => Box::new(std::io::stdout()),
        };
        Ok(Self { sink, colorize })
    }
}
