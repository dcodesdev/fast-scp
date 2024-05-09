use scp_rs::run::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
