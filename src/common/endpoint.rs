//! Provider-agnostic webhook endpoint builder.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Serialize, Serializer};

use crate::error::{Error, Result};
use crate::provider::Provider;

use super::emit;
use super::format::EmitFormat;
use super::signature::SignatureEncoding;

fn serialize_provider<S: Serializer>(
    provider: &Provider,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    serializer.serialize_str(provider.as_str())
}

/// Finished webhook endpoint description (URL + events + signature metadata).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Webhook {
    #[serde(serialize_with = "serialize_provider")]
    provider: Provider,
    url: String,
    events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_env: Option<String>,
    signature: SignatureMeta,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SignatureMeta {
    scheme: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<&'static str>,
}

impl Webhook {
    /// Provider id string (e.g. `stripe`).
    #[must_use]
    pub fn provider(&self) -> &'static str {
        self.provider.as_str()
    }

    /// Typed provider for delivery / registration routing.
    #[must_use]
    pub fn provider_kind(&self) -> Provider {
        self.provider
    }

    /// Delivery URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Subscribed event names in registration order (deduped).
    #[must_use]
    pub fn events(&self) -> &[String] {
        &self.events
    }

    /// Optional human description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Optional provider API version (Stripe, etc.).
    #[must_use]
    pub fn api_version(&self) -> Option<&str> {
        self.api_version.as_deref()
    }

    /// Env var name expected to hold the signing secret (never the secret itself).
    #[must_use]
    pub fn secret_env(&self) -> Option<&str> {
        self.secret_env.as_deref()
    }

    /// Signature scheme for this endpoint.
    #[must_use]
    pub fn signature_scheme(&self) -> &'static str {
        self.signature.scheme
    }

    /// Extra provider-specific string metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Pretty JSON document describing this webhook.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// KDL document describing this webhook (preferred source form).
    pub fn to_kdl(&self) -> Result<String> {
        Ok(emit::webhook_to_kdl(self))
    }

    /// Write JSON or KDL to `path`.
    pub fn write(&self, path: impl AsRef<Path>, format: EmitFormat) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|source| Error::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        let body = match format {
            EmitFormat::Json => self.to_json()?,
            EmitFormat::Kdl => self.to_kdl()?,
        };
        std::fs::write(path, body).map_err(|source| Error::Io {
            path: path.to_path_buf(),
            source,
        })
    }
}

/// Fluent builder for a single provider webhook endpoint.
#[derive(Debug, Clone)]
pub struct WebhookBuilder {
    provider: Provider,
    url: Option<String>,
    events: Vec<String>,
    description: Option<String>,
    api_version: Option<String>,
    secret_env: Option<String>,
    metadata: BTreeMap<String, String>,
}

impl WebhookBuilder {
    /// Start a builder for `provider` with that provider's default signature scheme.
    #[must_use]
    pub fn new(provider: Provider) -> Self {
        Self {
            provider,
            url: None,
            events: Vec::new(),
            description: None,
            api_version: None,
            secret_env: None,
            metadata: BTreeMap::new(),
        }
    }

    /// HTTPS (or http for local) delivery URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Subscribe to one event name (provider-canonical string).
    #[must_use]
    pub fn event(mut self, event: impl Into<String>) -> Self {
        let event = event.into();
        if !self.events.iter().any(|e| e == &event) {
            self.events.push(event);
        }
        self
    }

    /// Subscribe to many event names (strings or typed provider events).
    #[must_use]
    pub fn events<I, S>(mut self, events: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for event in events {
            self = self.event(event.as_ref());
        }
        self
    }

    /// Human-readable description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Provider API version pin (e.g. Stripe `2024-11-20.acacia`).
    #[must_use]
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = Some(version.into());
        self
    }

    /// Name of the environment variable that will hold the signing secret.
    ///
    /// Secrets themselves are never stored in the emitted config.
    #[must_use]
    pub fn secret_env(mut self, name: impl Into<String>) -> Self {
        self.secret_env = Some(name.into());
        self
    }

    /// Attach arbitrary string metadata (content-type hints, shop domain, etc.).
    #[must_use]
    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Validate and freeze the endpoint description.
    pub fn build(self) -> Result<Webhook> {
        let provider = self.provider.as_str();
        let url = self
            .url
            .filter(|u| !u.trim().is_empty())
            .ok_or(Error::WebhookMissingField {
                provider,
                field: "url",
            })?;

        validate_url(provider, &url)?;

        if self.events.is_empty() {
            return Err(Error::WebhookEmptyEvents { provider });
        }

        let scheme = self.provider.signature_scheme();
        Ok(Webhook {
            provider: self.provider,
            url,
            events: self.events,
            description: self.description,
            api_version: self.api_version,
            secret_env: self.secret_env,
            signature: SignatureMeta {
                scheme: scheme.as_str(),
                header: scheme.header(),
                encoding: scheme.encoding().map(SignatureEncoding::as_str),
            },
            metadata: self.metadata,
        })
    }
}

fn validate_url(provider: &'static str, url: &str) -> Result<()> {
    let url = url.trim();
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err(Error::WebhookInvalidUrl {
            provider,
            reason: "must start with http:// or https://",
        });
    }
    if url.len() < 10 {
        return Err(Error::WebhookInvalidUrl {
            provider,
            reason: "URL is too short",
        });
    }
    Ok(())
}
