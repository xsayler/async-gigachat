use anyhow::Ok;
use async_gigachat::{client::Client, config::GigaChatConfig, model::Models};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::default();

    let client = Client::with_config(config);

    let response = Models::new(client).list().await?;

    println!("models: {:?}", response.data);

    Ok(())
}
