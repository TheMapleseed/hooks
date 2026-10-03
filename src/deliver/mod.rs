//! Outbound webhook delivery: build → sign → POST.

mod payload;

use std::time::Duration;

use reqwest::blocking::Client as BlockingClient;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use uuid::Uuid;

use crate::common::Webhook;
use crate::error::{Error, Result};
use crate::provider::Provider;
use crate::secret::Secret;
use crate::sign::{sign, SignRequest};

pub use payload::{envelope_github, envelope_slack_event, envelope_stripe, OutboundEvent};

/// A ready-to-send webhook delivery.
#[derive(Debug, Clone)]
pub struct Delivery {
    /// Destination URL (overrides [`Webhook::url`] when set on the builder).
    pub url: String,
    /// Provider that owns the signing scheme / envelope.
    pub provider: Provider,
    /// Event / topic name placed in provider-specific headers or envelopes.
    pub event: String,
    /// Raw HTTP body.
    pub body: Vec<u8>,
    /// Optional Standard Webhooks message id.
    pub message_id: Option<String>,
}

impl Delivery {
    /// Start a delivery builder for `provider`.
    #[must_use]
    pub fn builder(provider: Provider) -> DeliveryBuilder {
        DeliveryBuilder::new(provider)
    }

    /// Build a delivery targeting a configured [`Webhook`] endpoint.
    pub fn to_webhook(webhook: &Webhook, event: impl AsRef<str>, body: impl AsRef<[u8]>) -> Self {
        Self {
            url: webhook.url().to_owned(),
            provider: webhook.provider_kind(),
            event: event.as_ref().to_owned(),
            body: body.as_ref().to_vec(),
            message_id: None,
        }
    }
}

/// Fluent builder for [`Delivery`].
#[derive(Debug, Clone)]
pub struct DeliveryBuilder {
    provider: Provider,
    url: Option<String>,
    event: Option<String>,
    body: Option<Vec<u8>>,
    message_id: Option<String>,
}

impl DeliveryBuilder {
    /// Create a builder for `provider`.
    #[must_use]
    pub fn new(provider: Provider) -> Self {
        Self {
            provider,
            url: None,
            event: None,
            body: None,
            message_id: None,
        }
    }

    /// Destination receiver URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Copy URL (and provider) from a configured webhook endpoint.
    #[must_use]
    pub fn webhook(mut self, webhook: &Webhook) -> Self {
        self.provider = webhook.provider_kind();
        self.url = Some(webhook.url().to_owned());
        self
    }

    /// Event / topic name.
    #[must_use]
    pub fn event(mut self, event: impl AsRef<str>) -> Self {
        self.event = Some(event.as_ref().to_owned());
        self
    }

    /// Raw body bytes.
    #[must_use]
    pub fn body(mut self, body: impl AsRef<[u8]>) -> Self {
        self.body = Some(body.as_ref().to_vec());
        self
    }

    /// JSON body (serialized compactly for stable signatures).
    pub fn json(mut self, value: &Value) -> Result<Self> {
        self.body = Some(serde_json::to_vec(value)?);
        Ok(self)
    }

    /// Standard Webhooks / Svix message id.
    #[must_use]
    pub fn message_id(mut self, id: impl Into<String>) -> Self {
        self.message_id = Some(id.into());
        self
    }

    /// Validate and freeze the delivery.
    pub fn build(self) -> Result<Delivery> {
        let provider = self.provider.as_str();
        let url = self.url.filter(|u| !u.trim().is_empty()).ok_or(
            Error::WebhookMissingField {
                provider,
                field: "url",
            },
        )?;
        let event = self.event.filter(|e| !e.trim().is_empty()).ok_or(
            Error::WebhookMissingField {
                provider,
                field: "event",
            },
        )?;
        let body = self.body.ok_or(Error::WebhookMissingField {
            provider,
            field: "body",
        })?;
        Ok(Delivery {
            url,
            provider: self.provider,
            event,
            body,
            message_id: self.message_id,
        })
    }
}

/// Result of an outbound delivery attempt.
#[derive(Debug, Clone)]
pub struct DeliveryResponse {
    /// HTTP status code from the receiver.
    pub status: u16,
    /// Response body (truncated for safety in logs by the caller).
    pub body: String,
    /// Final URL after redirects (as reported by reqwest).
    pub url: String,
}

/// Blocking HTTP client that signs and delivers webhooks.
#[derive(Debug, Clone)]
pub struct DeliveryClient {
    http: BlockingClient,
}

impl Default for DeliveryClient {
    fn default() -> Self {
        Self::new()
    }
}

impl DeliveryClient {
    /// Create a client with secure TLS defaults and a 30s timeout.
    pub fn new() -> Self {
        let http = BlockingClient::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(concat!("hooks-rs/", env!("CARGO_PKG_VERSION")))
            .https_only(false) // allow http://localhost in development
            .build()
            .expect("reqwest client");
        Self { http }
    }

    /// Sign `delivery` with `secret` and POST it to the destination.
    pub fn send(&self, delivery: &Delivery, secret: &Secret) -> Result<DeliveryResponse> {
        let timestamp = time::OffsetDateTime::now_utc().unix_timestamp();
        let message_id = delivery
            .message_id
            .clone()
            .unwrap_or_else(|| format!("msg_{}", Uuid::new_v4().simple()));

        let signed = sign(SignRequest {
            scheme: delivery.provider.signature_scheme(),
            body: &delivery.body,
            secret,
            timestamp,
            message_id: Some(message_id.as_str()),
            destination_url: Some(delivery.url.as_str()),
        })?;

        let mut headers = HeaderMap::new();
        for (name, value) in &signed.headers {
            let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|_| {
                Error::DeliveryFailed {
                    reason: format!("invalid header name: {name}"),
                }
            })?;
            let header_value = HeaderValue::from_str(value).map_err(|_| Error::DeliveryFailed {
                reason: format!("invalid header value for {name}"),
            })?;
            headers.insert(header_name, header_value);
        }

        // Provider event headers (in addition to signature headers).
        match delivery.provider {
            Provider::GitHub => {
                headers.insert(
                    HeaderName::from_static("x-github-event"),
                    HeaderValue::from_str(&delivery.event).map_err(|_| {
                        Error::DeliveryFailed {
                            reason: "invalid X-GitHub-Event value".to_owned(),
                        }
                    })?,
                );
            }
            Provider::Shopify => {
                headers.insert(
                    HeaderName::from_static("x-shopify-topic"),
                    HeaderValue::from_str(&delivery.event).map_err(|_| {
                        Error::DeliveryFailed {
                            reason: "invalid X-Shopify-Topic value".to_owned(),
                        }
                    })?,
                );
            }
            Provider::GitLab => {
                headers.insert(
                    HeaderName::from_static("x-gitlab-event"),
                    HeaderValue::from_str(&delivery.event).map_err(|_| {
                        Error::DeliveryFailed {
                            reason: "invalid X-Gitlab-Event value".to_owned(),
                        }
                    })?,
                );
            }
            _ => {}
        }

        let response = self
            .http
            .post(&delivery.url)
            .headers(headers)
            .body(delivery.body.clone())
            .send()
            .map_err(|err| Error::DeliveryFailed {
                reason: err.to_string(),
            })?;

        let status = response.status().as_u16();
        let url = response.url().to_string();
        let body = response.text().unwrap_or_default();

        if !(200..300).contains(&status) {
            return Err(Error::DeliveryRejected {
                status,
                body: truncate(&body, 512),
            });
        }

        Ok(DeliveryResponse { status, body, url })
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_owned()
    } else {
        format!("{}…", &s[..max])
    }
}
