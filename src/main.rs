use pie::runner::run;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    run().await
}
