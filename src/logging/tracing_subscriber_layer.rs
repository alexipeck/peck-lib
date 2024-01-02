use chrono::Utc;
use reqwest::Client;
use serde::Serialize;
use tokio::sync::mpsc::{self, Sender};
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

use super::log::Level;

pub trait MessageFromString {
    fn new(message: String) -> Self;
}

#[derive(Debug, Serialize)]
pub struct TeamsMessage {
    pub text: String,
}

impl MessageFromString for TeamsMessage {
    fn new(message: String) -> Self {
        TeamsMessage { text: message }
    }
}

#[derive(Debug, Serialize)]
pub struct DiscordMessage {
    pub content: String,
}

impl MessageFromString for DiscordMessage {
    fn new(message: String) -> Self {
        DiscordMessage { content: message }
    }
}

pub struct TracingLayer {
    sender: Sender<String>,
    level_filter: Level,
}

impl TracingLayer {
    pub async fn new<T>(uri: String, level_filter: Level) -> Self
    where
        T: MessageFromString + Serialize + Send + Sync + 'static,
    {
        let (sender, mut receiver) = mpsc::channel::<String>(128);

        let _ = tokio::spawn(async move {
            let client = Client::new();
            while let Some(message) = receiver.recv().await {
                match client.post(&uri).json(&T::new(message)).send().await {
                    Ok(response) => {
                        if response.status() != 200 {
                            eprintln!(
                                "Webhook returned non-200 status code \'{}\': {}",
                                response.status(),
                                response.text().await.unwrap_or("".into())
                            );
                        }
                    }
                    Err(err) => {
                        eprintln!("Error sending message via webhook \'{err}\'");
                    }
                };
            }
        });
        TracingLayer {
            sender,
            level_filter,
        }
    }

    fn send(&self, message: String) {
        let _ = self.sender.try_send(message);
    }
}

impl<S> Layer<S> for TracingLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let meta = event.metadata();

        let level = Level::from(meta.level());
        if level < self.level_filter {
            return;
        }

        let file = meta.file().unwrap_or_default();
        let line = match meta.line() {
            Some(line) => format!(":{}", line),
            None => String::new(),
        };
        let location = format!("{}{}", file, line);
        let mut content = String::new();
        {
            let mut visitor = StringVisitor(&mut content);
            event.record(&mut visitor);
        }
        self.send(format!(
            "{}\t{}\t{}\t{}",
            Utc::now().to_rfc3339(),
            level,
            location,
            content
        ));
    }
}

struct StringVisitor<'a>(&'a mut String);

impl<'a> tracing::field::Visit for StringVisitor<'a> {
    fn record_f64(&mut self, _field: &tracing::field::Field, _value: f64) {}

    fn record_i64(&mut self, _field: &tracing::field::Field, _value: i64) {}

    fn record_u64(&mut self, _field: &tracing::field::Field, _value: u64) {}

    fn record_bool(&mut self, _field: &tracing::field::Field, _value: bool) {}

    fn record_str(&mut self, _field: &tracing::field::Field, _value: &str) {}

    fn record_error(
        &mut self,
        _field: &tracing::field::Field,
        _value: &(dyn std::error::Error + 'static),
    ) {
    }

    fn record_debug(&mut self, _field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.push_str(&format!("{:?}", value));
    }
}
