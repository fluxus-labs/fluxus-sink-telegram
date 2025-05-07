use fluxus::sinks::{ConsoleFormatter, DefaultFormatter, Sink};
use fluxus::utils::models::{Record, StreamError};
use fluxus_sink_telegram::TelegramSink;
use std::time::SystemTime;

fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[tokio::test]
async fn test_telegram_sink_initialization() {
    let token = "test_token".to_string();
    let recipient = "test_recipient".to_string();
    let sink = TelegramSink::<String>::new(token, recipient, None);
    assert!(sink.is_ok());
}

#[tokio::test]
async fn test_telegram_sink_with_custom_formatter() {
    struct CustomFormatter;
    impl<T> ConsoleFormatter<T> for CustomFormatter {
        fn format(&self, _record: &Record<T>) -> String {
            "custom formatted message".to_string()
        }
    }

    let sink = TelegramSink::<String, CustomFormatter>::with_formatter(CustomFormatter);
    assert!(!sink.is_initialized());
}

#[tokio::test]
async fn test_telegram_sink_write() {
    let token = "test_token".to_string();
    let recipient = "test_recipient".to_string();
    let mut sink = TelegramSink::<String>::new(token, recipient, None).unwrap();

    let record = Record {
        data: "test message".to_string(),
        timestamp: current_time(),
    };

    let result = sink.write(record).await;
    assert!(result.is_err());
    match result {
        Err(StreamError::Runtime(_)) => (),
        _ => panic!("Expected Runtime error"),
    }
}

#[tokio::test]
async fn test_telegram_sink_uninitialized() {
    let mut sink = TelegramSink::<String, DefaultFormatter>::with_formatter(DefaultFormatter);

    let record = Record {
        data: "test message".to_string(),
        timestamp: current_time(),
    };

    let result = sink.write(record).await;
    assert!(result.is_err());
    match result {
        Err(StreamError::Config(_)) => (),
        _ => panic!("Expected Config error"),
    }
}
