use std::io::Write;
use reqwest::Response;
use anyhow::Result;
use crate::cli::CliArgs;

mod body;
mod headers;
mod highlight;

#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    Body,
    Headers,
    Both,
}

impl From<&CliArgs> for OutputMode {
    fn from(args: &CliArgs) -> Self {
        match (args.show_headers, args.head) {
            (_, true) => OutputMode::Headers,
            (true, false) => OutputMode::Both,
            (false, false) => OutputMode::Body,
        }
    }
}

pub async fn write_response(
    out: &mut dyn Write,
    colorize: bool,
    mode: OutputMode,
    response: Response,
) -> Result<()> {
    match mode {
        OutputMode::Body => body::write(out, colorize, response).await?,
        OutputMode::Headers => headers::write(out, colorize, &response)?,
        OutputMode::Both => {
            headers::write(out, colorize, &response)?;
            writeln!(out)?;
            body::write(out, colorize, response).await?;
        },
    }
    out.flush()?;
    Ok(())
}
