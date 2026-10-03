//! Lightweight provider envelopes for outbound test/synthetic events.
//!
//! These are programmable JSON builders — not embedded static templates.

use serde_json::{json, Value};
use time::OffsetDateTime;
use uuid::Uuid;

/// A structured outbound event before serialization.
#[derive(Debug, Clone)]
pub struct OutboundEvent {
    /// Provider event / topic name.
    pub event: String,
    /// Event payload object (provider `data.object`, Slack event, etc.).
    pub data: Value,
}

impl OutboundEvent {
    /// Create an outbound event.
    #[must_use]
    pub fn new(event: impl Into<String>, data: Value) -> Self {
        Self {
            event: event.into(),
            data,
        }
    }
}

/// Wrap `event` in a Stripe Event object envelope.
#[must_use]
pub fn envelope_stripe(event: &OutboundEvent) -> Value {
    let id = format!("evt_test_{}", Uuid::new_v4().simple());
    let created = OffsetDateTime::now_utc().unix_timestamp();
    json!({
        "id": id,
        "object": "event",
        "api_version": "2024-11-20.acacia",
        "created": created,
        "type": event.event,
        "livemode": false,
        "pending_webhooks": 1,
        "request": { "id": null, "idempotency_key": null },
        "data": { "object": event.data },
    })
}

/// GitHub deliveries use the raw resource payload; event name goes in a header.
#[must_use]
pub fn envelope_github(event: &OutboundEvent) -> Value {
    event.data.clone()
}

/// Wrap `event` in a Slack Events API `event_callback` envelope.
#[must_use]
pub fn envelope_slack_event(event: &OutboundEvent, team_id: &str) -> Value {
    json!({
        "token": "verification-token-placeholder",
        "team_id": team_id,
        "api_app_id": "A0TEST",
        "type": "event_callback",
        "event_id": format!("Ev{}", Uuid::new_v4().simple()),
        "event_time": OffsetDateTime::now_utc().unix_timestamp(),
        "event": {
            "type": event.event,
            "data": event.data,
        }
    })
}
