use scp_cli::run::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
