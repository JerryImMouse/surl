use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::Response;
use std::io::Write;

pub async fn write(out: &mut dyn Write, colorize: bool, response: Response) -> Result<()> {
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let syntax = super::highlight::syntax_for(content_type);

    if let Some(syntax) = syntax && colorize
    {
        // because of how syntect handles highlighting I need to download whole body
        // here
        let body = response.text().await.context("failed to get body")?;
        super::highlight::highlight(out, &body, syntax)?;
    } else {
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("failed to read response chunk")?;
            out.write_all(&chunk)?;
        }
    }
    Ok(())
}
