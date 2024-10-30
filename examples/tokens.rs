use async_gigachat::{
    client::Client,
    config::GigaChatConfig,
    result::Result,
    token::{TokenCountRequestBuilder, Tokens},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::new();

    let client = Client::with_config(config);

    let request = TokenCountRequestBuilder::default()
        .model("GigaChat:latest")
        .input(vec![
            "Why shouldn't you leave the room?".to_owned(),
            "What is the mistake?".to_owned(),
        ])
        .build()?;

    let response = Tokens::new(client).get_token_count(request).await?;

    println!("response: {:?}", response);

    Ok(())
}
