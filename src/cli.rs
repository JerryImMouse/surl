use anyhow::Result;
use clap::Parser;
use reqwest::Method;
use std::path::PathBuf;
use url::Url;

use crate::{
    client,
    output::Output,
    render::{self, OutputMode},
    request::{self, RequestSpec},
};

#[derive(Debug, Parser)]
#[clap(long_about = None, about, version)]
pub struct CliArgs {
    /// Url to request to
    pub url: Url,

    /// Define a custom header <header/@file_of_headers>
    #[arg(short = 'H')]
    pub header: Vec<String>,

    /// Follow redirects
    #[arg(short = 'L')]
    pub location: bool,

    /// Show response headers
    #[arg(short = 'i')]
    pub show_headers: bool,

    /// Send a custom User-Agent to host
    #[arg(short = 'A', default_value = "surl/0.1.0")]
    pub user_agent: String,

    /// user:password basic auth
    #[arg(short = 'u')]
    pub user: Option<String>,

    #[arg(short = 'X')]
    pub request: Option<Method>,

    /// Redirect output from stdout to a file
    #[arg(short = 'o')]
    pub output: Option<PathBuf>,

    /// TODO
    #[arg(short = 'I')]
    pub head: bool,

    /// By default: sURL automatically determines the response body format
    /// and highlights it if its supported, can be memory heavy due to
    /// because we need to get the whole response to style it.
    /// When this flag is provided the above is turned off and a memory-less method is used
    #[arg(short = 'M')]
    pub no_highlight: bool,

    /// Provide a POST data for a request it can be either a file path or plain text or `-` to
    /// use input from stdin
    #[arg(short = 'd')]
    pub data: Option<String>,
}

pub async fn run() -> Result<()> {
    let args = CliArgs::parse();

    let spec = RequestSpec::try_from(&args)?;
    let client = client::build_client(&args)?;
    let mode = OutputMode::from(&args);
    let mut output = Output::new(args.output, args.no_highlight)?;

    let response = request::fetch(&client, spec).await?;

    render::write_response(&mut output.sink, output.colorize, mode, response).await
}
