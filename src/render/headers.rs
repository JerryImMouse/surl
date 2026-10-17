use anyhow::Result;
use colored::Colorize;
use reqwest::Response;
use std::io::Write;

pub fn write(out: &mut dyn Write, colorize: bool, response: &Response) -> Result<()> {
    let status = response.status();
    let version = format!("{:?}", response.version());

    if colorize {
        let status_colored = if status.is_success() {
            status.to_string().green().bold()
        } else if status.is_redirection() {
            status.to_string().yellow().bold()
        } else {
            status.to_string().red().bold()
        };
        writeln!(out, "{} {}", version.dimmed(), status_colored)?;

        for (k, v) in response.headers() {
            let v = String::from_utf8_lossy(v.as_bytes());
            writeln!(out, "{}: {}", k.as_str().cyan(), v)?;
        }
    } else {
        writeln!(out, "{:?} {}", version, status)?;
        for (k, v) in response.headers() {
            let v = String::from_utf8_lossy(v.as_bytes());
            writeln!(out, "{}: {}", k, v)?;
        }
    }

    Ok(())
}
