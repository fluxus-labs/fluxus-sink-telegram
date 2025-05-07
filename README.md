# Fluxus Sink Telegram

A Telegram sink component for the Fluxus stream processing framework, enabling real-time message delivery to Telegram channels and chats.

## Features

- Send messages to Telegram channels and chats
- Support for both channel usernames and chat IDs
- Optional proxy configuration for network connectivity
- Asynchronous message delivery
- Error handling and retry mechanisms
- Easy integration with Fluxus framework

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fluxus-sink-telegram = "0.1"
```

## Usage

### Basic Example

```rust
use fluxus_sink_telegram::TelegramSink;
use fluxus::sinks::Sink;
use fluxus::utils::models::Record;
use std::time::SystemTime;

fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the Telegram sink
    let mut sink = TelegramSink::new(
        "YOUR_BOT_TOKEN".to_string(),
        "@your_channel".to_string(),
        None, // Optional proxy
    )?;

    let record = Record {
        data: "Hello from Fluxus!".to_string(),
        timestamp: current_time(),
    };

    // Send a message
    sink.write(record).await?;

    Ok(())
}
```

### Using with Proxy

```rust
let mut sink = TelegramSink::new(
    "YOUR_BOT_TOKEN".to_string(),
    "@your_channel".to_string(),
    Some("http://proxy.example.com:8080".to_string()),
)?;
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
