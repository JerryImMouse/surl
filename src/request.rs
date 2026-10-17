use anyhow::{Context, Result};
use reqwest::{Client, Method, Response, header::HeaderMap};
use url::Url;

use crate::{body::BodySource, cli::CliArgs};

#[derive(Debug)]
pub struct RequestSpec {
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
    pub basic: Option<(String, String)>,
    pub body: Option<Vec<u8>>,
}

impl TryFrom<&CliArgs> for RequestSpec {
    type Error = anyhow::Error;

    fn try_from(args: &CliArgs) -> Result<Self, Self::Error> {
        let method = match (&args.request, args.head, &args.data) {
            (Some(method), _, _) => method.clone(),
            (None, true, _) => Method::HEAD,
            (None, false, Some(_)) => Method::POST,
            (None, false, None) => Method::GET,
        };

        let basic = args
            .user
            .as_deref()
            .map(crate::utils::parse_basic_auth)
            .transpose()?;
        let headers = crate::utils::build_headers(args.header.clone())?;

        let body = args
            .data
            .as_deref()
            .map(BodySource::parse)
            .map(BodySource::read)
            .transpose()?;

        Ok(Self {
            method,
            url: args.url.clone(),
            headers,
            basic,
            body,
        })
    }
}

pub async fn fetch(client: &Client, spec: RequestSpec) -> Result<Response> {
    let mut request = client.request(spec.method, spec.url).headers(spec.headers);

    if let Some((user, pass)) = spec.basic {
        request = request.basic_auth(user, Some(pass));
    }

    if let Some(body) = spec.body {
        request = request.body(body);
    }

    request.send().await.context("failed to send request")
}
