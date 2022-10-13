use super::env::ConfigLog;
use std::collections::BTreeMap;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Layer};

pub fn init(cfg: ConfigLog) -> tracing_appender::non_blocking::WorkerGuard {
    let env_log_level = cfg.level;

    let with_file = true;
    let with_line_number = true;
    let with_target = false;
    let log_level = match env_log_level.to_lowercase().as_str() {
        "error" => (
            tracing::Level::ERROR,
            tracing_subscriber::filter::LevelFilter::ERROR,
        ),
        "warn" => (
            tracing::Level::WARN,
            tracing_subscriber::filter::LevelFilter::WARN,
        ),
        "debug" => (
            tracing::Level::DEBUG,
            tracing_subscriber::filter::LevelFilter::DEBUG,
        ),
        "trace" => (
            tracing::Level::TRACE,
            tracing_subscriber::filter::LevelFilter::TRACE,
        ),
        _ => (
            tracing::Level::INFO,
            tracing_subscriber::filter::LevelFilter::INFO,
        ),
    };

    let log_file_dir = cfg.file;
    let (non_blocking, guard) = if log_file_dir.is_empty() {
        tracing_appender::non_blocking(std::io::stdout())
    } else {
        let file_appender = tracing_appender::rolling::daily(".log", log_file_dir);
        tracing_appender::non_blocking(file_appender)
    };

    if cfg.is_json {
        let layer = CustomJsonLayer;
        tracing_subscriber::registry()
            .with(layer.with_filter(log_level.1))
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_file(with_file)
            .with_line_number(with_line_number)
            .with_target(with_target)
            .with_max_level(log_level.0)
            .with_writer(non_blocking)
            .init();
    }

    tracing::debug!("Start tracing on {} level", log_level.0);
    guard
}

struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<'a> tracing::field::Visit for JsonVisitor<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }
}

pub struct CustomJsonLayer;

#[derive(Debug)]
struct CustomSpanField(BTreeMap<String, serde_json::Value>);

impl<S> Layer<S> for CustomJsonLayer
where
    S: tracing::Subscriber + for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut spans = vec![];
        let event_scope = ctx.event_scope(event);

        if event_scope.is_some() {
            let scope = event_scope.unwrap();
            for span in scope.from_root() {
                let extensions = span.extensions();
                let ext_data = extensions.get::<CustomSpanField>();
                if ext_data.is_some() {
                    let storage = ext_data.unwrap();
                    let field_data: &BTreeMap<String, serde_json::Value> = &storage.0;
                    for field in field_data.into_iter() {
                        spans.push(serde_json::json!({ field.0: field.1 }));
                    }
                }
            }
        }

        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        let output = serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "file": format!("{}:{}", event.metadata().file().unwrap_or_default(), event.metadata().line().unwrap_or_default()),
            "level": event.metadata().level().to_string(),
            "message": fields
                .get("message")
                .unwrap_or(&serde_json::Value::default()),
            "context": spans,
        });
        println!("{}", serde_json::to_string(&output).unwrap());
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        attrs.record(&mut visitor);

        let storage = CustomSpanField(fields);

        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        extensions.insert::<CustomSpanField>(storage);
    }
}
