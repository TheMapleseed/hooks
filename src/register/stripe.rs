//! Create Stripe webhook endpoints via the Stripe API.

use std::collections::HashMap;
use std::time::Duration;

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::common::Webhook;
use crate::error::{Error, Result};
use crate::provider::Provider;
use crate::secret::Secret;

use super::{RegisteredEndpoint, Registrar};

/// Stripe API registrar (`POST /v1/webhook_endpoints`).
#[derive(Debug, Clone)]
pub struct StripeRegistrar {
    api_key: Secret,
    http: Client,
    api_base: String,
}

impl StripeRegistrar {
    /// Create a registrar using a Stripe secret API key (`sk_…`).
    pub fn new(api_key: Secret) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(concat!("hooks-rs/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("reqwest client");
        Self {
            api_key,
            http,
            api_base: "https://api.stripe.com".to_owned(),
        }
    }

    /// Override API base (tests / Stripe mocks).
    #[must_use]
    pub fn api_base(mut self, base: impl Into<String>) -> Self {
        self.api_base = base.into();
        self
    }
}

/// Stripe `webhook_endpoint` create response (subset).
#[derive(Debug, Deserialize)]
struct StripeWebhookEndpoint {
    id: String,
    url: String,
    status: String,
    enabled_events: Vec<String>,
    #[serde(default)]
    secret: Option<String>,
    #[serde(default)]
    api_version: Option<String>,
}

/// Convenience alias for the Stripe registration result.
pub type StripeRegistration = RegisteredEndpoint;

impl Registrar for StripeRegistrar {
    fn register(&self, webhook: &Webhook) -> Result<RegisteredEndpoint> {
        if webhook.provider_kind() != Provider::Stripe {
            return Err(Error::RegisterProviderMismatch {
                expected: Provider::Stripe.as_str(),
                actual: webhook.provider(),
            });
        }

        let mut form: HashMap<String, String> = HashMap::new();
        form.insert("url".to_owned(), webhook.url().to_owned());
        for (idx, event) in webhook.events().iter().enumerate() {
            form.insert(format!("enabled_events[{idx}]"), event.clone());
        }
        if let Some(description) = webhook.description() {
            form.insert("description".to_owned(), description.to_owned());
        }
        if let Some(api_version) = webhook.api_version() {
            form.insert("api_version".to_owned(), api_version.to_owned());
        }

        let response = self
            .http
            .post(format!("{}/v1/webhook_endpoints", self.api_base.trim_end_matches('/')))
            .basic_auth(self.api_key.expose(), None::<&str>)
            .form(&form)
            .send()
            .map_err(|err| Error::RegisterFailed {
                provider: Provider::Stripe.as_str(),
                reason: err.to_string(),
            })?;

        let status = response.status();
        let text = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(Error::RegisterFailed {
                provider: Provider::Stripe.as_str(),
                reason: format!("HTTP {status}: {}", truncate(&text, 512)),
            });
        }

        let created: StripeWebhookEndpoint =
            serde_json::from_str(&text).map_err(|err| Error::RegisterFailed {
                provider: Provider::Stripe.as_str(),
                reason: format!("decode response: {err}"),
            })?;

        let mut details = vec![("status".to_owned(), created.status)];
        if let Some(api_version) = created.api_version {
            details.push(("api_version".to_owned(), api_version));
        }

        Ok(RegisteredEndpoint {
            provider: Provider::Stripe.as_str(),
            id: Some(created.id),
            url: created.url,
            events: created.enabled_events,
            signing_secret: created.secret,
            details,
        })
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_owned()
    } else {
        format!("{}…", &s[..max])
    }
}
