//! # hooks
//!
//! Swiss-army-knife crate for common **HTTP webhooks**:
//!
//! 1. **Describe** — typed builders (no embedded templates)
//! 2. **Register** — create endpoints via provider APIs (Stripe, GitHub, …)
//! 3. **Deliver** — sign and POST payloads to the correct destination
//!
//! ## Describe + deliver (Stripe)
//!
//! ```rust,no_run
//! use hooks::deliver::{envelope_stripe, Delivery, DeliveryClient, OutboundEvent};
//! use hooks::stripe::{self, Event as StripeEvent};
//! use hooks::Secret;
//! use serde_json::json;
//!
//! let endpoint = stripe::endpoint()
//!     .url("https://api.example.com/webhooks/stripe")
//!     .events([StripeEvent::PaymentIntentSucceeded])
//!     .secret_env("STRIPE_WEBHOOK_SECRET")
//!     .build()
//!     .unwrap();
//!
//! let event = OutboundEvent::new(
//!     StripeEvent::PaymentIntentSucceeded.as_str(),
//!     json!({"id": "pi_123", "object": "payment_intent", "amount": 1000}),
//! );
//! let body = serde_json::to_vec(&envelope_stripe(&event)).unwrap();
//! let delivery = Delivery::to_webhook(&endpoint, event.event, body);
//! let secret = Secret::from_env("STRIPE_WEBHOOK_SECRET").unwrap();
//! let _response = DeliveryClient::new().send(&delivery, &secret).unwrap();
//! ```
//!
//! ## Register with Stripe
//!
//! ```rust,no_run
//! use hooks::register::{Registrar, StripeRegistrar};
//! use hooks::stripe::{self, Event as StripeEvent};
//! use hooks::Secret;
//!
//! let webhook = stripe::endpoint()
//!     .url("https://api.example.com/webhooks/stripe")
//!     .events([StripeEvent::CheckoutSessionCompleted])
//!     .build()
//!     .unwrap();
//!
//! let registered = StripeRegistrar::new(Secret::from_env("STRIPE_API_KEY").unwrap())
//!     .register(&webhook)
//!     .unwrap();
//! println!("created {}", registered.id.unwrap());
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod client;
pub mod common;
pub mod conformance;
pub mod deliver;
pub mod provider;
pub mod providers;
pub mod register;
pub mod secret;
pub mod sign;

mod error;

pub use client::Client;
pub use common::{EmitFormat, SignatureEncoding, SignatureScheme, Webhook, WebhookBuilder};
pub use conformance::{assert_suite_passes, run_suite, ConformanceReport};
pub use deliver::{
    envelope_github, envelope_slack_event, envelope_stripe, Delivery, DeliveryBuilder,
    DeliveryClient, DeliveryResponse, OutboundEvent,
};
pub use error::{Error, Result};
pub use provider::Provider;
pub use register::{GitHubRegistrar, RegisteredEndpoint, Registrar, StripeRegistrar};
pub use secret::Secret;
pub use sign::{sign, sign_headers, SignRequest, SignedRequest};

pub use providers::{
    bitbucket, clerk, discord, github, gitlab, hubspot, linear, pagerduty, paypal, sendgrid,
    shopify, slack, standard_webhooks, stripe, twilio, vercel,
};
