use chrono::Utc;
use reqwest::blocking::Client;
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

use super::log::Level;

pub struct TeamsChannelLayer {
    uri: &'static str,
    client: Client,
}

impl TeamsChannelLayer {
    pub fn new(uri: &'static str) -> Self {
        TeamsChannelLayer {
            uri,
            client: Client::new(),
        }
    }

    fn send(&self, message: &str) {
        let response = match self.client.post(self.uri).json(&message).send() {
            Ok(response) => response,
            Err(err) => {
                eprintln!(
                    "Error sending message via Teams webhook \'{err}\': {}",
                    message
                );
                return;
            }
        };
        if response.status() != 200 {
            eprintln!(
                "Teams webhook returned non-200 status code \'{}\': {}",
                response.status(),
                response.text().unwrap_or("".into())
            );
        }
    }
}

impl<S> Layer<S> for TeamsChannelLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let meta = event.metadata();
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
        let message = &format!(
            "{}\t{}\t{}\t{}",
            Utc::now().to_rfc3339(),
            Level::from(meta.level()),
            location,
            content
        );
        self.send(message);
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
