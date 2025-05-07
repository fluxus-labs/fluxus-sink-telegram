//! Fluxus Sink Telegram
//!
//! A Telegram sink component for the Fluxus stream processing framework that enables
//! real-time message delivery to Telegram channels and chats. This component provides
//! a simple and efficient way to send messages to Telegram as part of your data
//! processing pipeline.
//!
//! # Features
//!
//! - Send messages to Telegram channels and chats
//! - Support for both channel usernames and chat IDs
//! - Optional proxy configuration for network connectivity
//! - Asynchronous message delivery
//! - Error handling and retry mechanisms
//!
//! # Example
//!
//! ```rust,no_run
//! use fluxus_sink_telegram::TelegramSink;
//! use fluxus::sinks::Sink;
//! use fluxus::utils::models::Record;
//! use std::time::SystemTime;
//!
//! fn current_time() -> i64 {
//!     SystemTime::now()
//!         .duration_since(SystemTime::UNIX_EPOCH)
//!         .unwrap()
//!         .as_secs() as i64
//! }
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut sink = TelegramSink::new(
//!         "YOUR_BOT_TOKEN".to_string(),
//!         "@your_channel".to_string(),
//!         None, // Optional proxy
//!     )?;
//!
//!     let record = Record {
//!         data: "Hello from Fluxus!".to_string(),
//!         timestamp: current_time(),
//!     };
//!
//!     sink.write(record).await?;
//!
//!     Ok(())
//! }
//! ```

mod telegram_bot;

mod telegram;
pub use telegram::TelegramSink;
