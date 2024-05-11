use fast_scp::run::run;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
