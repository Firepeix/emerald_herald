use crate::date::DateTime;

use super::storage::JsonStorage;
use serde::ser::{SerializeMap, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use tracing::{Event, Id, Subscriber, Metadata};
use tracing_core::metadata::Level;
use tracing_core::span::Attributes;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::SpanRef;
use tracing_subscriber::Layer;


const TIMESTAMP: &str = "timestamp";
const LEVEL: &str = "level";
const MESSAGE: &str = "message";
const HOSTNAME: &str = "hostname";
const LOGGER: &str = "logger";
const SERVICE: &str = "service";
const CONTEXT: &str = "context";

const RESERVED_FIELDS: [&str; 7] = [TIMESTAMP, LEVEL, MESSAGE, HOSTNAME, LOGGER, CONTEXT, SERVICE];


/// This layer is exclusively concerned with formatting information.
/// It relies on the upstream `JsonStorageLayer` to get access to the fields attached to
/// each span.
pub struct FormattingLayer<W: MakeWriter> {
    make_writer: W,
    hostname: String,
    service: String,
    default_fields: HashMap<String, Value>,
}

impl<W: MakeWriter> FormattingLayer<W> {
    /// Create a new `FormattingLayer`.
    ///
    /// You have to specify:
    /// - a `service`, which will be attached to all formatted records
    /// - a `make_writer`, which will be used to get a `Write` instance to write formatted records to.
    ///
    /// ## Using stdout
    /// ```rust
    /// use formatter::FormattingLayer;
    ///
    /// let formatting_layer = FormattingLayer::new("tracing_example".into(), std::io::stdout);
    /// ```
    ///
    /// If you prefer, you can use closure syntax:
    /// ```rust
    /// use formatter::FormattingLayer;
    ///
    /// let formatting_layer = FormattingLayer::new("tracing_example".into(), || std::io::stdout());
    /// ```
    pub fn new(service: String, make_writer: W) -> Self {
        Self::with_default_fields(service, make_writer, HashMap::new())
    }

    pub fn with_default_fields(service: String, make_writer: W, default_fields: HashMap<String, Value>) -> Self {
        Self {
            make_writer,
            service,
            hostname: gethostname::gethostname().to_string_lossy().into_owned(),
            default_fields,
        }
    }

    fn serialize_core_fields(
        &self,
        map_serializer: &mut impl SerializeMap<Error = serde_json::Error>,
        message: &str,
        level: &Level,
        logger: &str
    ) -> Result<(), std::io::Error> {

        map_serializer.serialize_entry(TIMESTAMP, &DateTime::now())?;
        map_serializer.serialize_entry(LEVEL, level.as_str())?;
        map_serializer.serialize_entry(MESSAGE, &message)?;
        map_serializer.serialize_entry(HOSTNAME, &self.hostname)?;
        map_serializer.serialize_entry(SERVICE, &self.service)?;
        map_serializer.serialize_entry(LOGGER, logger)?;
       
        Ok(())
    }

    fn serialize_context(&self, map_serializer: &mut impl SerializeMap<Error = serde_json::Error>, context: &HashMap<&str, Value> ) -> Result<(), std::io::Error> {

        let context: HashMap<String, Value> = context.clone().iter()
        .filter(|(&key, _)| key != "message" && !RESERVED_FIELDS.contains(&key))
        .map(|(key, value)| (key.to_string(), value.clone()))
        .collect();

        if !context.is_empty() {
            map_serializer.serialize_entry(CONTEXT, &context)?;
        }
       
        Ok(())
    }

    /// Given a span, it serialised it to a in-memory buffer (vector of bytes).
    fn serialize_span<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
        &self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut buffer);
        let mut map_serializer = serializer.serialize_map(None)?;
        let message = format_span_context(span, ty);
        self.serialize_core_fields(&mut map_serializer, &message, span.metadata().level(), span.metadata().target())?;
        
        // Add all default fields
        for (key, value) in self.default_fields.iter() {
            if !RESERVED_FIELDS.contains(&key.as_str()) {
                map_serializer.serialize_entry(key, value)?;
            } else {
                tracing::debug!(
                        "{} is a reserved field in the bunyan log format. Skipping it.",
                        key
                    );
            }
        }

        let extensions = span.extensions();
        if let Some(visitor) = extensions.get::<JsonStorage>() {
            self.serialize_context(&mut map_serializer, visitor.values())?
        }
        map_serializer.end()?;
        Ok(buffer)
    }

    /// Given an in-memory buffer holding a complete serialised record, flush it to the writer
    /// returned by self.make_writer.
    ///
    /// We add a trailing new-line at the end of the serialised record.
    ///
    /// If we write directly to the writer returned by self.make_writer in more than one go
    /// we can end up with broken/incoherent bits and pieces of those records when
    /// running multi-threaded/concurrent programs.
    fn emit(&self, mut buffer: Vec<u8>, meta: &Metadata<'_>) -> Result<(), std::io::Error> {
        buffer.write_all(b"\n")?;
        self.make_writer.make_writer_for(meta).write_all(&buffer)
    }
}

/// The type of record we are dealing with: entering a span, exiting a span, an event.
#[derive(Clone, Debug)]
pub enum Type {
    EnterSpan,
    ExitSpan,
    Event,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Type::EnterSpan => "START",
            Type::ExitSpan => "END",
            Type::Event => "EVENT",
        };
        write!(f, "{}", repr)
    }
}

/// Ensure consistent formatting of the span context.
///
/// Example: "[AN_INTERESTING_SPAN - START]"
fn format_span_context<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
    span: &SpanRef<S>,
    ty: Type,
) -> String {
    format!("[{} - {}]", span.metadata().name().to_uppercase(), ty)
}

/// Ensure consistent formatting of event message.
///
/// Examples:
/// - "[AN_INTERESTING_SPAN - EVENT] My event message" (for an event with a parent span)
/// - "My event message" (for an event without a parent span)
fn format_event_message<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(
    current_span: &Option<SpanRef<S>>,
    event: &Event,
    event_visitor: &JsonStorage<'_>,
) -> String {
    // Extract the "message" field, if provided. Fallback to the target, if missing.
    let mut message = event_visitor
        .values()
        .get("message")
        .and_then(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or_else(|| event.metadata().target())
        .to_owned();

    // If the event is in the context of a span, prepend the span name to the message.
    if let Some(span) = &current_span {
        message = format!("{} {}", format_span_context(span, Type::Event), message);
    }

    message
}

impl<S, W> Layer<S> for FormattingLayer<W>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    W: for<'a> MakeWriter + 'static,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Events do not necessarily happen in the context of a span, hence lookup_current
        // returns an `Option<SpanRef<_>>` instead of a `SpanRef<_>`.
        let current_span = ctx.lookup_current();

        let mut event_visitor = JsonStorage::default();
        event.record(&mut event_visitor);

        // Opting for a closure to use the ? operator and get more linear code.
        let format = || {
            let mut buffer = Vec::new();

            let mut serializer = serde_json::Serializer::new(&mut buffer);
            let mut map_serializer = serializer.serialize_map(None)?;

            let message = format_event_message(&current_span, event, &event_visitor);
            self.serialize_core_fields(
                &mut map_serializer,
                &message,
                event.metadata().level(),
                event.metadata().target()
            )?;

            // Add all default fields
            for (key, value) in self.default_fields
                .iter()
                .filter(|(key, _)| key.as_str() != "message" && !RESERVED_FIELDS.contains(&key.as_str()))
            {
                map_serializer.serialize_entry(key, value)?;
            }

            // Add all the other fields associated with the event, expect the message we already used.
            self.serialize_context(&mut map_serializer, event_visitor.values())?;

            // Add all the fields from the current span, if we have one.
            if let Some(span) = &current_span {
                let extensions = span.extensions();
                if let Some(visitor) = extensions.get::<JsonStorage>() {
                    for (key, value) in visitor.values() {
                        if !RESERVED_FIELDS.contains(key) {
                            map_serializer.serialize_entry(key, value)?;
                        } else {
                            tracing::debug!(
                                "{} is a reserved field in the bunyan log format. Skipping it.",
                                key
                            );
                        }
                    }
                }
            }
            map_serializer.end()?;
            Ok(buffer)
        };

        let result: std::io::Result<Vec<u8>> = format();
        if let Ok(formatted) = result {
            let _ = self.emit(formatted, event.metadata());
        }
    }

    fn new_span(&self, _attrs: &Attributes, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        if let Ok(serialized) = self.serialize_span(&span, Type::EnterSpan) {
            let _ = self.emit(serialized, span.metadata());
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        if let Ok(serialized) = self.serialize_span(&span, Type::ExitSpan) {
            let _ = self.emit(serialized, span.metadata());
        }
    }
}