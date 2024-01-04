<h1> async-gigachat </h1>
<p> Async Rust library for GigaChat </p>

## Overview

`async-gigachat` is an unofficial Rust library for GigaChat REST API.

## Usage

The library reads [Authorization token](https://developers.sber.ru/docs/ru/gigachat/api/authorization) from the environment variable `GIGACHAT_AUTH_TOKEN`.

```bash
# On macOS/Linux
export GIGACHAT_AUTH_TOKEN='YTAxNj...'
```

```powershell
# On Windows Powershell
$Env:GIGACHAT_AUTH_TOKEN='YTAxNj...'
```

- Visit [examples](https://github.com/xsayler/async-gigachat/tree/main/examples) directory on how to use `async-gigachat`.
- Visit [docs.rs/async-gigachat](https://docs.rs/async-gigachat) for docs.

## Chat completion Example

```rust
use anyhow::Ok;
use async_gigachat::{
    chat::{ChatCompletionRequestBuilder, ChatMessageBuilder, Role, Chat},
    client::Client,
    config::GigaChatConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = GigaChatConfig::default();

    let client: Client = Client::with_config(config);

    let question = ChatMessageBuilder::default()
    .role(Role::User)
    .content("Hey, how's it going?")
    .build()?;

    let request = ChatCompletionRequestBuilder::default()
        .messages(vec![question.clone()])
        .model("GigaChat:latest")
        .build()?;

    let response = Chat::new(client).completion(request).await?;

    println!("{}: {}", question.role, question.content);
    println!("{}: {}", response.choices.get(0).unwrap().message.role, response.choices.get(0).unwrap().message.content);

    Ok(())
}
```
## License

This project is licensed under [MIT license](https://github.com/xsayler/async-gigachat/blob/main/LICENSE).
