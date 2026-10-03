use hooks::deliver::{Delivery, DeliveryClient};
use hooks::register::{Registrar, StripeRegistrar};
use hooks::sign::{sign, SignRequest};
use hooks::{Provider, Secret};
use serde_json::json;
use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn signs_github_hmac_sha256() {
    let secret = Secret::new("it's a secret");
    let body = br#"{"zen":"Design for failure."}"#;
    let signed = sign(SignRequest::for_provider(
        Provider::GitHub,
        body,
        &secret,
        0,
    ))
    .expect("sign");

    let header = signed.headers.get("X-Hub-Signature-256").expect("header");
    assert!(header.starts_with("sha256="));
    assert_eq!(header.len(), "sha256=".len() + 64);
}

#[test]
fn signs_stripe_timestamped_header() {
    let secret = Secret::new("whsec_test");
    let body = br#"{"id":"evt_1"}"#;
    let signed = sign(SignRequest::for_provider(
        Provider::Stripe,
        body,
        &secret,
        1_600_000_000,
    ))
    .expect("sign");
    let header = signed.headers.get("Stripe-Signature").expect("header");
    assert!(header.contains("t=1600000000"));
    assert!(header.contains("v1="));
}

#[tokio::test]
async fn delivers_signed_post_to_receiver() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/hook"))
        .and(header_exists("Stripe-Signature"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let url = format!("{}/hook", server.uri());
    let secret = Secret::new("test_secret");
    let body = br#"{"type":"payment_intent.succeeded"}"#;
    let delivery = Delivery::builder(Provider::Stripe)
        .url(url)
        .event("payment_intent.succeeded")
        .body(body)
        .build()
        .expect("delivery");

    // blocking client inside async test — acceptable for this integration check
    let response = tokio::task::spawn_blocking(move || {
        DeliveryClient::new().send(&delivery, &secret)
    })
    .await
    .expect("join")
    .expect("send");

    assert_eq!(response.status, 200);
    assert_eq!(response.body, "ok");
}

#[tokio::test]
async fn registers_stripe_webhook_endpoint() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/webhook_endpoints"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "we_123",
            "url": "https://api.example.com/webhooks/stripe",
            "status": "enabled",
            "enabled_events": ["checkout.session.completed"],
            "secret": "whsec_abc",
            "api_version": "2024-11-20.acacia"
        })))
        .mount(&server)
        .await;

    let webhook = hooks::stripe::endpoint()
        .url("https://api.example.com/webhooks/stripe")
        .events([hooks::stripe::Event::CheckoutSessionCompleted])
        .api_version("2024-11-20.acacia")
        .build()
        .expect("webhook");

    let api_base = server.uri();
    let registered = tokio::task::spawn_blocking(move || {
        StripeRegistrar::new(Secret::new("sk_test_123"))
            .api_base(api_base)
            .register(&webhook)
    })
    .await
    .expect("join")
    .expect("register");

    assert_eq!(registered.id.as_deref(), Some("we_123"));
    assert_eq!(registered.signing_secret.as_deref(), Some("whsec_abc"));
    assert_eq!(registered.events, vec!["checkout.session.completed"]);
}
