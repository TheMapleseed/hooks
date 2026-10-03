//! HTTP webhook providers with typed event surfaces.
//!
//! Each module exposes:
//! - a typed [`Event`](stripe::Event) enum (provider-canonical wire names)
//! - [`endpoint`](stripe::endpoint) → [`WebhookBuilder`](crate::common::WebhookBuilder)
//! - [`webhook`](stripe::webhook) convenience constructor

#[allow(unused_imports)]
#[macro_use]
mod macros;

pub mod bitbucket;
pub mod clerk;
pub mod discord;
pub mod github;
pub mod gitlab;
pub mod hubspot;
pub mod linear;
pub mod pagerduty;
pub mod paypal;
pub mod sendgrid;
pub mod shopify;
pub mod slack;
pub mod standard;
pub mod stripe;
pub mod twilio;
pub mod vercel;

/// Re-export Standard Webhooks under the longer name used by [`crate::Provider`].
pub use standard as standard_webhooks;
