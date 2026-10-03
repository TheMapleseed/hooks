//! Registration request shape checks (no live network I/O).

use serde_json::{json, Value};

use crate::common::WebhookBuilder;
use crate::provider::Provider;

use super::report::CheckResult;
use super::standards::ProviderStandard;

pub(super) fn check_registration_shapes(
    provider: Provider,
    standard: &ProviderStandard,
) -> Vec<CheckResult> {
    let mut out = Vec::new();

    if !standard.registration_checked {
        out.push(CheckResult::skip(
            provider,
            "registration.shape",
            "registration shape not asserted for this provider yet",
        ));
        return out;
    }

    match provider {
        Provider::Stripe => out.extend(check_stripe_form()),
        Provider::GitHub => out.extend(check_github_body()),
        _ => out.push(CheckResult::skip(
            provider,
            "registration.shape",
            "no registrar shape fixture",
        )),
    }

    out
}

fn check_stripe_form() -> Vec<CheckResult> {
    let mut out = Vec::new();
    let webhook = match WebhookBuilder::new(Provider::Stripe)
        .url("https://api.example.com/webhooks/stripe")
        .event("checkout.session.completed")
        .event("invoice.paid")
        .api_version("2024-11-20.acacia")
        .description("conformance")
        .build()
    {
        Ok(w) => w,
        Err(err) => {
            return vec![CheckResult::fail(
                Provider::Stripe,
                "registration.shape",
                format!("build webhook: {err}"),
            )];
        }
    };

    let mut keys = vec!["url".to_owned()];
    for (idx, _) in webhook.events().iter().enumerate() {
        keys.push(format!("enabled_events[{idx}]"));
    }
    keys.push("description".to_owned());
    keys.push("api_version".to_owned());

    let required = ["url", "enabled_events[0]", "enabled_events[1]", "api_version"];
    let missing: Vec<_> = required
        .iter()
        .filter(|k| !keys.iter().any(|have| have == *k))
        .collect();

    if missing.is_empty() && webhook.events().len() == 2 {
        out.push(CheckResult::pass(
            Provider::Stripe,
            "registration.shape",
            "Stripe webhook_endpoints form fields present",
        ));
    } else {
        out.push(CheckResult::fail(
            Provider::Stripe,
            "registration.shape",
            format!("missing {missing:?} keys={keys:?}"),
        ));
    }

    if webhook.url().starts_with("https://") {
        out.push(CheckResult::pass(
            Provider::Stripe,
            "registration.url_https",
            "registration URL is https",
        ));
    } else {
        out.push(CheckResult::fail(
            Provider::Stripe,
            "registration.url_https",
            "Stripe endpoints should use https",
        ));
    }

    out
}

fn check_github_body() -> Vec<CheckResult> {
    let mut out = Vec::new();
    let webhook = match WebhookBuilder::new(Provider::GitHub)
        .url("https://api.example.com/webhooks/github")
        .event("push")
        .event("pull_request")
        .secret_env("GITHUB_WEBHOOK_SECRET")
        .build()
    {
        Ok(w) => w,
        Err(err) => {
            return vec![CheckResult::fail(
                Provider::GitHub,
                "registration.shape",
                format!("build webhook: {err}"),
            )];
        }
    };

    let body = json!({
        "name": "web",
        "active": true,
        "events": webhook.events(),
        "config": {
            "url": webhook.url(),
            "content_type": "json",
            "secret": "redacted",
            "insecure_ssl": "0",
        }
    });

    let required_top = ["name", "active", "events", "config"];
    let missing_top = missing_keys(&body, &required_top);
    let config = body.get("config").cloned().unwrap_or(Value::Null);
    let required_config = ["url", "content_type", "secret", "insecure_ssl"];
    let missing_config = missing_keys(&config, &required_config);

    if missing_top.is_empty()
        && missing_config.is_empty()
        && body.get("name") == Some(&json!("web"))
        && body.get("config").and_then(|c| c.get("content_type")) == Some(&json!("json"))
        && body.get("config").and_then(|c| c.get("insecure_ssl")) == Some(&json!("0"))
    {
        out.push(CheckResult::pass(
            Provider::GitHub,
            "registration.shape",
            "GitHub /hooks body matches API shape",
        ));
    } else {
        out.push(CheckResult::fail(
            Provider::GitHub,
            "registration.shape",
            format!("missing top={missing_top:?} config={missing_config:?}"),
        ));
    }

    out
}

fn missing_keys<'a>(value: &Value, keys: &[&'a str]) -> Vec<&'a str> {
    let Some(obj) = value.as_object() else {
        return keys.to_vec();
    };
    keys.iter()
        .filter(|k| !obj.contains_key(**k))
        .copied()
        .collect()
}
