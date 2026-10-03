//! Describe → sign → (optional) deliver for Stripe + GitHub.
//!
//! ```sh
//! cargo run --example swiss_army
//! HOOKS_DEMO_URL=http://127.0.0.1:9999/hook cargo run --example swiss_army
//! ```

use std::process::ExitCode;

use hooks::deliver::{envelope_stripe, Delivery, DeliveryClient, OutboundEvent};
use hooks::sign::{sign, SignRequest};
use hooks::stripe::{self, Event as StripeEvent};
use hooks::{github, Provider, Secret};
use serde_json::json;

fn main() -> ExitCode {
    let secret = Secret::new("test_signing_secret_value");
    let body = br#"{"ok":true}"#;
    let signed = match sign(SignRequest::for_provider(
        Provider::GitHub,
        body,
        &secret,
        1_700_000_000,
    )) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("sign: {err}");
            return ExitCode::FAILURE;
        }
    };
    println!(
        "github signature = {:?}",
        signed.headers.get("X-Hub-Signature-256")
    );

    let endpoint = match stripe::endpoint()
        .url("https://example.invalid/webhooks/stripe")
        .events([StripeEvent::PaymentIntentSucceeded])
        .build()
    {
        Ok(e) => e,
        Err(err) => {
            eprintln!("stripe endpoint: {err}");
            return ExitCode::FAILURE;
        }
    };

    let event = OutboundEvent::new(
        StripeEvent::PaymentIntentSucceeded.as_str(),
        json!({"id": "pi_demo", "object": "payment_intent", "amount": 4200}),
    );
    let envelope = envelope_stripe(&event);
    println!(
        "stripe envelope type = {}",
        envelope.get("type").and_then(|v| v.as_str()).unwrap_or("?")
    );
    println!("configured delivery target = {}", endpoint.url());

    let gh = match github::endpoint()
        .url("https://example.invalid/webhooks/github")
        .events([github::Event::Push, github::Event::PullRequest])
        .secret_env("GITHUB_WEBHOOK_SECRET")
        .build()
    {
        Ok(w) => w,
        Err(err) => {
            eprintln!("github endpoint: {err}");
            return ExitCode::FAILURE;
        }
    };
    println!("github events = {:?}", gh.events());

    if let Ok(url) = std::env::var("HOOKS_DEMO_URL") {
        let body = match serde_json::to_vec(&envelope) {
            Ok(b) => b,
            Err(err) => {
                eprintln!("json: {err}");
                return ExitCode::FAILURE;
            }
        };
        let delivery = match Delivery::builder(Provider::Stripe)
            .url(url)
            .event(StripeEvent::PaymentIntentSucceeded.as_str())
            .body(body)
            .build()
        {
            Ok(d) => d,
            Err(err) => {
                eprintln!("delivery: {err}");
                return ExitCode::FAILURE;
            }
        };
        match DeliveryClient::new().send(&delivery, &secret) {
            Ok(resp) => println!("delivered HTTP {}", resp.status),
            Err(err) => {
                eprintln!("delivery: {err}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        println!("set HOOKS_DEMO_URL to POST a live signed Stripe-shaped event");
    }

    ExitCode::SUCCESS
}
