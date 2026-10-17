use std::str::FromStr;

use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn build_headers(h: Vec<String>) -> Result<HeaderMap<HeaderValue>> {
    let parts = h
        .into_iter()
        .map(|s| {
            if let Some((key, val)) = s.trim().split_once(":") {
                Ok((
                    HeaderName::from_str(key.trim())
                        .with_context(|| format!("invalid header: {}", s))?,
                    HeaderValue::from_str(val.trim())
                        .with_context(|| format!("invalid header: {}", s))?,
                ))
            } else {
                Err(anyhow::anyhow!("invalid header: {s}"))
            }
        })
        .collect::<Vec<Result<(HeaderName, HeaderValue)>>>();

    let mut header_map = HeaderMap::new();
    for part in parts {
        let (key, val) = part?;
        header_map.insert(key, val);
    }

    Ok(header_map)
}

pub fn parse_basic_auth(d: &str) -> Result<(String, String)> {
    let (user, password) = d.split_once(":").context("invalid basic auth")?;
    Ok((user.to_owned(), password.to_owned()))
}
