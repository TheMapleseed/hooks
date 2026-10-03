//! Emit sample webhook descriptions for several HTTP providers.
//!
//! ```sh
//! cargo run --example catalog -- ./out
//! ```

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use hooks::github::{self, Event as GitHubEvent};
use hooks::shopify::{self, Event as ShopifyEvent};
use hooks::slack::{self, Event as SlackEvent};
use hooks::stripe::{self, Event as StripeEvent};
use hooks::{EmitFormat, Provider};

fn main() -> ExitCode {
    let out = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("out"));

    if let Err(err) = std::fs::create_dir_all(&out) {
        eprintln!("mkdir {}: {err}", out.display());
        return ExitCode::FAILURE;
    }

    println!(
        "providers: {:?}",
        Provider::all()
            .iter()
            .map(|p| p.as_str())
            .collect::<Vec<_>>()
    );

    let samples = [
        (
            "stripe",
            stripe::endpoint()
                .url("https://api.example.com/webhooks/stripe")
                .events([
                    StripeEvent::PaymentIntentSucceeded,
                    StripeEvent::CheckoutSessionCompleted,
                    StripeEvent::CustomerSubscriptionUpdated,
                    StripeEvent::InvoicePaid,
                ])
                .api_version("2024-11-20.acacia")
                .secret_env("STRIPE_WEBHOOK_SECRET")
                .build(),
        ),
        (
            "github",
            github::endpoint()
                .url("https://api.example.com/webhooks/github")
                .events([
                    GitHubEvent::Push,
                    GitHubEvent::PullRequest,
                    GitHubEvent::Issues,
                    GitHubEvent::WorkflowRun,
                ])
                .secret_env("GITHUB_WEBHOOK_SECRET")
                .build(),
        ),
        (
            "slack",
            slack::endpoint()
                .url("https://api.example.com/webhooks/slack")
                .events([
                    SlackEvent::AppMention,
                    SlackEvent::Message,
                    SlackEvent::UrlVerification,
                ])
                .secret_env("SLACK_SIGNING_SECRET")
                .build(),
        ),
        (
            "shopify",
            shopify::endpoint()
                .url("https://api.example.com/webhooks/shopify")
                .events([
                    ShopifyEvent::OrdersCreate,
                    ShopifyEvent::OrdersPaid,
                    ShopifyEvent::AppUninstalled,
                ])
                .secret_env("SHOPIFY_WEBHOOK_SECRET")
                .meta("shop", "example.myshopify.com")
                .build(),
        ),
    ];

    for (name, result) in samples {
        let webhook = match result {
            Ok(w) => w,
            Err(err) => {
                eprintln!("{name}: {err}");
                return ExitCode::FAILURE;
            }
        };
        let path = out.join(format!("{name}.webhook.kdl"));
        if let Err(err) = webhook.write(&path, EmitFormat::Kdl) {
            eprintln!("write {}: {err}", path.display());
            return ExitCode::FAILURE;
        }
        println!("wrote {}", path.display());
    }

    ExitCode::SUCCESS
}
