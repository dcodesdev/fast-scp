use dotenvy::dotenv;
use scp_cli::run::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    run().await
}
