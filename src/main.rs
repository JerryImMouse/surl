#[tokio::main]
async fn main() {
    if let Err(e) = surl::cli::run().await {
        eprintln!("{e}");
    }
}
