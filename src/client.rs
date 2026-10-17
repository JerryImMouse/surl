use anyhow::{Context, Result};
use reqwest::{Client, redirect::Policy};

use crate::cli::CliArgs;

pub fn build_client(args: &CliArgs) -> Result<Client> {
    let mut builder = Client::builder().user_agent(&args.user_agent);

    if !args.location {
        builder = builder.redirect(Policy::none());
    }

    builder
        .build()
        .context("failed to build a client")
}
