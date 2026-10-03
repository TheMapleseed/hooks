//! Outbound payload / envelope shape checks.

use serde_json::{json, Value};

use crate::deliver::{envelope_github, envelope_slack_event, envelope_stripe, OutboundEvent};
use crate::provider::Provider;

use super::report::CheckResult;
use super::standards::ProviderStandard;

pub(super) fn check_payloads(
    provider: Provider,
    standard: &ProviderStandard,
) -> Vec<CheckResult> {
    let mut out = Vec::new();

    if standard.payload_required_keys.is_empty() && standard.payload_object.is_none() {
        out.push(CheckResult::skip(
            provider,
            "payload.envelope",
            "no envelope shape required for this provider",
        ));
        return out;
    }

    let payload = match build_payload(provider) {
        Some(p) => p,
        None => {
            out.push(CheckResult::skip(
                provider,
                "payload.envelope",
                "no synthetic envelope builder for this provider",
            ));
            return out;
        }
    };

    let Some(obj) = payload.as_object() else {
        out.push(CheckResult::fail(
            provider,
            "payload.envelope",
            "payload is not a JSON object",
        ));
        return out;
    };

    let mut missing = Vec::new();
    for key in standard.payload_required_keys {
        if !obj.contains_key(*key) {
            missing.push(*key);
        }
    }
    if missing.is_empty() {
        out.push(CheckResult::pass(
            provider,
            "payload.required_keys",
            format!("has {:?}", standard.payload_required_keys),
        ));
    } else {
        out.push(CheckResult::fail(
            provider,
            "payload.required_keys",
            format!("missing keys {missing:?}"),
        ));
    }

    if let Some(expected_object) = standard.payload_object {
        match obj.get("object").and_then(Value::as_str) {
            Some(actual) if actual == expected_object => out.push(CheckResult::pass(
                provider,
                "payload.object",
                format!("object == {expected_object:?}"),
            )),
            Some(actual) => out.push(CheckResult::fail(
                provider,
                "payload.object",
                format!("object={actual:?} want {expected_object:?}"),
            )),
            None => out.push(CheckResult::fail(
                provider,
                "payload.object",
                "missing object field",
            )),
        }
    }

    // Stripe-specific: data.object must exist
    if provider == Provider::Stripe {
        match payload.pointer("/data/object") {
            Some(_) => out.push(CheckResult::pass(
                provider,
                "payload.data.object",
                "Stripe data.object present",
            )),
            None => out.push(CheckResult::fail(
                provider,
                "payload.data.object",
                "Stripe envelope missing data.object",
            )),
        }
        if payload.get("type").and_then(Value::as_str) == Some("payment_intent.succeeded") {
            out.push(CheckResult::pass(
                provider,
                "payload.type",
                "type mirrors outbound event name",
            ));
        } else {
            out.push(CheckResult::fail(
                provider,
                "payload.type",
                format!("unexpected type {:?}", payload.get("type")),
            ));
        }
    }

    if provider == Provider::Slack {
        match payload.get("type").and_then(Value::as_str) {
            Some("event_callback") => out.push(CheckResult::pass(
                provider,
                "payload.type",
                "Slack event_callback envelope",
            )),
            other => out.push(CheckResult::fail(
                provider,
                "payload.type",
                format!("expected event_callback, got {other:?}"),
            )),
        }
    }

    out
}

fn build_payload(provider: Provider) -> Option<Value> {
    match provider {
        Provider::Stripe => {
            let event = OutboundEvent::new(
                "payment_intent.succeeded",
                json!({"id": "pi_conformance", "object": "payment_intent"}),
            );
            Some(envelope_stripe(&event))
        }
        Provider::Slack => {
            let event = OutboundEvent::new("app_mention", json!({"text": "hi"}));
            Some(envelope_slack_event(&event, "TCONFORM"))
        }
        Provider::GitHub => {
            let event = OutboundEvent::new("push", json!({"ref": "refs/heads/main"}));
            Some(envelope_github(&event))
        }
        _ => None,
    }
}
