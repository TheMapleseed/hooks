//! Crate-level errors. Kept small so callers only depend on stable failure modes.

use std::io;
use std::path::PathBuf;

/// Failures that can occur while building, signing, delivering, or registering webhooks.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Webhook endpoint is missing a required field.
    #[error("{provider} webhook is missing required field: {field}")]
    WebhookMissingField {
        /// Provider id (e.g. `stripe`).
        provider: &'static str,
        /// Name of the missing field.
        field: &'static str,
    },

    /// Webhook endpoint has no subscribed events.
    #[error("{provider} webhook must subscribe to at least one event")]
    WebhookEmptyEvents {
        /// Provider id (e.g. `stripe`).
        provider: &'static str,
    },

    /// Endpoint URL failed basic validation.
    #[error("{provider} webhook URL is invalid: {reason}")]
    WebhookInvalidUrl {
        /// Provider id (e.g. `stripe`).
        provider: &'static str,
        /// Why the URL was rejected.
        reason: &'static str,
    },

    /// Required secret environment variable was missing or empty.
    #[error("missing or empty secret environment variable `{name}`")]
    MissingSecretEnv {
        /// Environment variable name.
        name: String,
    },

    /// Signing is not implemented for this scheme (yet).
    #[error("signing is not supported for scheme `{scheme}`")]
    SigningUnsupported {
        /// Scheme id.
        scheme: &'static str,
    },

    /// Cryptographic signing failed.
    #[error("signing failed: {reason}")]
    SigningFailed {
        /// Why signing failed.
        reason: &'static str,
    },

    /// HTTP delivery transport failure.
    #[error("webhook delivery failed: {reason}")]
    DeliveryFailed {
        /// Transport or header error details.
        reason: String,
    },

    /// Receiver rejected the delivery with a non-2xx status.
    #[error("webhook delivery rejected with HTTP {status}: {body}")]
    DeliveryRejected {
        /// HTTP status code.
        status: u16,
        /// Truncated response body.
        body: String,
    },

    /// Provider registration API call failed.
    #[error("failed to register {provider} webhook: {reason}")]
    RegisterFailed {
        /// Provider id.
        provider: &'static str,
        /// API / decode error details.
        reason: String,
    },

    /// Registrar was used with a webhook for a different provider.
    #[error("registrar expected `{expected}` webhook, got `{actual}`")]
    RegisterProviderMismatch {
        /// Expected provider id.
        expected: &'static str,
        /// Actual provider id.
        actual: &'static str,
    },

    /// JSON serialization failed.
    #[error("failed to serialize config to JSON: {0}")]
    Json(#[from] serde_json::Error),

    /// Filesystem write or directory creation failed.
    #[error("failed to write config to `{path}`: {source}")]
    Io {
        /// Path that could not be written or created.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },
}

/// Convenient result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;
