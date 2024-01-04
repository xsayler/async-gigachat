use async_gigachat::{client::Client, config::GigaChatConfig, model::Models, result::Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::default();

    let client = Client::with_config(config);

    let response = Models::new(client).list().await?;

    println!("models: {:?}", response.data);

    Ok(())
}
