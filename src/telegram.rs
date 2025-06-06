use crate::telegram_bot::TelegramBot;
use async_trait::async_trait;
use fluxus::sinks::{ConsoleFormatter, DefaultFormatter, Sink};
use fluxus::utils::models::{Record, StreamError, StreamResult};
use std::marker::PhantomData;

/// A sink that writes to console
#[derive(Default, Clone)]
pub struct TelegramSink<T, F = DefaultFormatter> {
    formatter: F,
    bot: TelegramBot,
    _phantom: PhantomData<T>,
}

impl<T> TelegramSink<T, DefaultFormatter> {
    /// Create a new console sink with default formatter
    pub fn new(token: String, recipient: String, proxy: Option<String>) -> anyhow::Result<Self> {
        Ok(Self {
            bot: TelegramBot::new(token, recipient, proxy)?,
            formatter: DefaultFormatter,
            _phantom: PhantomData,
        })
    }
}

impl<T, F> TelegramSink<T, F> {
    /// Create a new console sink with custom formatter
    pub fn with_formatter(formatter: F) -> Self {
        Self {
            bot: TelegramBot::default(),
            formatter,
            _phantom: PhantomData,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.bot.is_initialized()
    }
}

#[async_trait]
impl<T, F> Sink<T> for TelegramSink<T, F>
where
    T: Send,
    F: ConsoleFormatter<T> + Send + Sync,
{
    async fn init(&mut self) -> StreamResult<()> {
        Ok(())
    }

    async fn write(&mut self, record: Record<T>) -> StreamResult<()> {
        if !self.bot.is_initialized() {
            return Err(StreamError::Config(
                "Telegram bot not initialized".to_string(),
            ));
        }
        let message = self.formatter.format(&record);
        self.bot
            .send_message(message)
            .await
            .map_err(|e| StreamError::Runtime(format!("Failed to send message: {}", e)))?;
        Ok(())
    }

    async fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    async fn close(&mut self) -> StreamResult<()> {
        Ok(())
    }
}
