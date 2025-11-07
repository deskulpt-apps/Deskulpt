//! Utilities for enriching tracing spans with Deskulpt-specific context.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;
use tracing::span::{Attributes, Id};
use tracing::{Span, Subscriber};
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry::LookupSpan;

/// Context about the widget that owns the current span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetContext {
    widget_id: Arc<str>,
    plugin_id: Option<Arc<str>>,
}

impl WidgetContext {
    /// Create a new [`WidgetContext`].
    pub fn new<S>(widget_id: S, plugin_id: Option<String>) -> Self
    where
        S: Into<String>,
    {
        Self {
            widget_id: Arc::<str>::from(widget_id.into()),
            plugin_id: plugin_id.map(|id| Arc::<str>::from(id)),
        }
    }

    /// Identifier of the widget that produced the span.
    pub fn widget_id(&self) -> &str {
        &self.widget_id
    }

    /// Optional plugin identifier that scoped the widget.
    pub fn plugin_id(&self) -> Option<&str> {
        self.plugin_id.as_deref()
    }
}

/// Context about a trigger currently being processed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TriggerContext {
    trigger_id: Arc<str>,
    widget: Option<WidgetContext>,
}

impl TriggerContext {
    /// Create a new [`TriggerContext`] with an optional widget association.
    pub fn new(trigger_id: impl Into<String>, widget: Option<WidgetContext>) -> Self {
        Self {
            trigger_id: Arc::<str>::from(trigger_id.into()),
            widget,
        }
    }

    /// Identifier of the trigger.
    pub fn trigger_id(&self) -> &str {
        &self.trigger_id
    }

    /// Widget that originated the trigger, when available.
    pub fn widget(&self) -> Option<&WidgetContext> {
        self.widget.as_ref()
    }

    /// Set the widget context for the trigger.
    pub fn with_widget(mut self, widget: WidgetContext) -> Self {
        self.widget = Some(widget);
        self
    }
}

/// Layer that tracks span relationships so we can store metadata externally.
pub struct SpanContextLayer;

impl SpanContextLayer {
    /// Create a new [`SpanContextLayer`].
    pub const fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for SpanContextLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let parent = attrs
            .parent()
            .cloned()
            .or_else(|| ctx.lookup_current().map(|span| span.id().clone()));
        SPAN_STORE.register_span(id, parent.as_ref());
    }

    fn on_close(&self, id: Id, _ctx: Context<'_, S>) {
        SPAN_STORE.remove_span(&id);
    }
}

/// Attach widget context to a span so structured logs can include widget
/// metadata.
pub fn attach_widget_context(span: &Span, widget_id: &str, plugin_id: Option<&str>) {
    if let Some(id) = span.id() {
        let context = WidgetContext::new(widget_id, plugin_id.map(|pid| pid.to_string()));
        SPAN_STORE.set_widget(&id, context);
    }
}

/// Attach trigger context to a span.
pub fn attach_trigger(span: &Span, trigger: TriggerContext) {
    if let Some(id) = span.id() {
        SPAN_STORE.set_trigger(&id, trigger);
    }
}

/// Retrieve the widget context associated with the current span, if any.
pub fn current_widget() -> Option<WidgetContext> {
    let id = Span::current().id()?.into_u64();
    SPAN_STORE.lookup_widget(id)
}

/// Retrieve the trigger context associated with the current span, if any.
pub fn current_trigger() -> Option<TriggerContext> {
    let id = Span::current().id()?.into_u64();
    SPAN_STORE.lookup_trigger(id)
}

static SPAN_STORE: Lazy<SpanStore> = Lazy::new(SpanStore::default);

#[derive(Default)]
struct SpanStore {
    spans: RwLock<HashMap<u64, SpanRecord>>,
}

#[derive(Clone, Default)]
struct SpanRecord {
    parent: Option<u64>,
    widget: Option<WidgetContext>,
    trigger: Option<TriggerContext>,
}

impl SpanStore {
    fn register_span(&self, id: &Id, parent: Option<&Id>) {
        let mut spans = self.spans.write().unwrap();
        spans
            .entry(id.into_u64())
            .and_modify(|record| record.parent = parent.map(|p| p.into_u64()))
            .or_insert_with(|| SpanRecord {
                parent: parent.map(|p| p.into_u64()),
                ..Default::default()
            });
    }

    fn remove_span(&self, id: &Id) {
        let mut spans = self.spans.write().unwrap();
        spans.remove(&id.into_u64());
    }

    fn set_widget(&self, id: &Id, ctx: WidgetContext) {
        let mut spans = self.spans.write().unwrap();
        spans.entry(id.into_u64()).or_default().widget = Some(ctx);
    }

    fn set_trigger(&self, id: &Id, ctx: TriggerContext) {
        let mut spans = self.spans.write().unwrap();
        spans.entry(id.into_u64()).or_default().trigger = Some(ctx);
    }

    fn lookup_widget(&self, start: u64) -> Option<WidgetContext> {
        self.lookup_from(start, |record| record.widget.clone())
    }

    fn lookup_trigger(&self, start: u64) -> Option<TriggerContext> {
        self.lookup_from(start, |record| record.trigger.clone())
    }

    fn lookup_from<T, F>(&self, start: u64, mut f: F) -> Option<T>
    where
        F: FnMut(&SpanRecord) -> Option<T>,
    {
        let spans = self.spans.read().unwrap();
        let mut current = Some(start);
        while let Some(id) = current {
            if let Some(record) = spans.get(&id) {
                if let Some(value) = f(record) {
                    return Some(value);
                }
                current = record.parent;
            } else {
                break;
            }
        }
        None
    }
}
