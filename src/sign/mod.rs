//! Provider-correct webhook request signing.
//!
//! This module only produces authenticity headers. It does not perform HTTP I/O.

mod schemes;

use std::collections::BTreeMap;

use crate::common::SignatureScheme;
use crate::error::{Error, Result};
use crate::provider::Provider;
use crate::secret::Secret;

pub use schemes::SignedRequest;

/// Inputs required to sign an outbound webhook body.
#[derive(Debug, Clone)]
pub struct SignRequest<'a> {
    /// Signature scheme to apply.
    pub scheme: SignatureScheme,
    /// Raw HTTP body bytes that will be delivered.
    pub body: &'a [u8],
    /// Shared signing secret.
    pub secret: &'a Secret,
    /// Unix timestamp used by time-windowed schemes (Stripe, Slack, Standard).
    pub timestamp: i64,
    /// Message id for Standard Webhooks / Svix (`msg_…`).
    pub message_id: Option<&'a str>,
    /// Full destination URL (required for Twilio-style signatures).
    pub destination_url: Option<&'a str>,
}

impl<'a> SignRequest<'a> {
    /// Build a sign request for a provider's default scheme.
    #[must_use]
    pub fn for_provider(
        provider: Provider,
        body: &'a [u8],
        secret: &'a Secret,
        timestamp: i64,
    ) -> Self {
        Self {
            scheme: provider.signature_scheme(),
            body,
            secret,
            timestamp,
            message_id: None,
            destination_url: None,
        }
    }

    /// Set the Standard Webhooks / Svix message id.
    #[must_use]
    pub fn message_id(mut self, id: &'a str) -> Self {
        self.message_id = Some(id);
        self
    }

    /// Set the destination URL (Twilio).
    #[must_use]
    pub fn destination_url(mut self, url: &'a str) -> Self {
        self.destination_url = Some(url);
        self
    }
}

/// Sign `request` and return headers that must be sent with the body.
pub fn sign(request: SignRequest<'_>) -> Result<SignedRequest> {
    match request.scheme {
        SignatureScheme::StripeSignature => schemes::stripe(&request),
        SignatureScheme::GitHubHmacSha256 | SignatureScheme::BitbucketHmacSha256 => {
            schemes::github_sha256(&request)
        }
        SignatureScheme::SlackSigningSecret => schemes::slack(&request),
        SignatureScheme::ShopifyHmacSha256 => schemes::shopify(&request),
        SignatureScheme::StandardWebhooks | SignatureScheme::Svix => schemes::standard_webhooks(&request),
        SignatureScheme::LinearHmacSha256 | SignatureScheme::VercelSignature => {
            schemes::hex_hmac_sha256_body(&request)
        }
        SignatureScheme::HubSpotV3 => schemes::hubspot_v3(&request),
        SignatureScheme::TwilioSignature => schemes::twilio(&request),
        SignatureScheme::GitLabToken => schemes::gitlab_token(&request),
        SignatureScheme::DiscordEd25519
        | SignatureScheme::PayPalTransmission
        | SignatureScheme::PagerDutySignature
        | SignatureScheme::IpAllowlist => Err(Error::SigningUnsupported {
            scheme: request.scheme.as_str(),
        }),
    }
}

/// Convenience: headers as an owned map for HTTP clients.
pub fn sign_headers(request: SignRequest<'_>) -> Result<BTreeMap<String, String>> {
    Ok(sign(request)?.headers)
}
