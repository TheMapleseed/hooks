use hooks::stripe::{self, Event as StripeEvent};
use hooks::{github, slack, EmitFormat, Error, Provider};

#[test]
fn stripe_webhook_programmatic() {
    let hook = stripe::endpoint()
        .url("https://api.example.com/webhooks/stripe")
        .events([
            StripeEvent::PaymentIntentSucceeded,
            StripeEvent::CheckoutSessionCompleted,
        ])
        .api_version("2024-11-20.acacia")
        .secret_env("STRIPE_WEBHOOK_SECRET")
        .build()
        .expect("stripe");

    assert_eq!(hook.provider(), "stripe");
    assert_eq!(hook.signature_scheme(), "stripe-signature");

    let json = hook.to_json().expect("json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("parse");
    assert_eq!(value["signature"]["header"], "Stripe-Signature");
    assert!(value["events"]
        .as_array()
        .unwrap()
        .iter()
        .any(|e| e == "payment_intent.succeeded"));

    let kdl = hook.to_kdl().expect("kdl");
    assert!(
        kdl.contains("provider=stripe") || kdl.contains("provider=\"stripe\""),
        "unexpected kdl: {kdl}"
    );
    assert!(kdl.contains("payment_intent.succeeded"));
}

#[test]
fn github_and_slack_require_events_and_url() {
    let err = github::endpoint()
        .url("https://api.example.com/hooks/github")
        .build()
        .unwrap_err();
    assert!(matches!(err, Error::WebhookEmptyEvents { .. }));

    let err = slack::endpoint()
        .events([slack::Event::AppMention])
        .build()
        .unwrap_err();
    assert!(matches!(
        err,
        Error::WebhookMissingField { field: "url", .. }
    ));
}

#[test]
fn provider_catalog_includes_stripe_and_github() {
    let ids: Vec<_> = Provider::all().iter().map(|p| p.as_str()).collect();
    assert!(ids.contains(&"stripe"));
    assert!(ids.contains(&"github"));
    assert!(ids.contains(&"shopify"));
    assert!(ids.contains(&"standard-webhooks"));
    assert_eq!(
        Provider::Stripe.signature_scheme().header(),
        Some("Stripe-Signature")
    );
}

#[test]
fn writes_webhook_kdl() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/hooks-webhook-write");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("mkdir");

    let hook = stripe::webhook(
        "https://api.example.com/webhooks/stripe",
        [StripeEvent::InvoicePaid],
    )
    .expect("webhook");

    let path = dir.join("stripe.kdl");
    hook.write(&path, EmitFormat::Kdl).expect("write");
    let body = std::fs::read_to_string(path).expect("read");
    assert!(body.contains("invoice.paid"));
}
