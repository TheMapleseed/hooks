//! Signature metadata for provider webhooks.
//!
//! This crate records *how* a provider signs deliveries so configs stay
//! self-describing. Signing implementations live in [`crate::sign`].

use serde::Serialize;

/// Cryptographic scheme a provider uses (or omits) for webhook authenticity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum SignatureScheme {
    /// Stripe `Stripe-Signature` (`t=…,v1=…` HMAC-SHA256).
    StripeSignature,
    /// GitHub `X-Hub-Signature-256` (`sha256=…`).
    GitHubHmacSha256,
    /// GitLab `X-Gitlab-Token` shared secret header.
    GitLabToken,
    /// Slack Events API (`v0=…` HMAC-SHA256 over timestamp + body).
    SlackSigningSecret,
    /// Discord Interactions Ed25519 (`X-Signature-Ed25519` + timestamp).
    DiscordEd25519,
    /// Shopify `X-Shopify-Hmac-Sha256` (base64 HMAC-SHA256).
    ShopifyHmacSha256,
    /// Twilio `X-Twilio-Signature` (HMAC-SHA1 over URL + params).
    TwilioSignature,
    /// Standard Webhooks (`webhook-id` / `webhook-timestamp` / `webhook-signature`).
    StandardWebhooks,
    /// Svix-compatible Standard Webhooks (Resend, Clerk, etc.).
    Svix,
    /// HubSpot `X-HubSpot-Signature-v3`.
    HubSpotV3,
    /// SendGrid/Twilio SendGrid — typically IP allowlist, not HMAC.
    IpAllowlist,
    /// Bitbucket Cloud HMAC (`X-Hub-Signature` sha256).
    BitbucketHmacSha256,
    /// Vercel deployment/log drains style HMAC.
    VercelSignature,
    /// Linear `Linear-Signature` HMAC-SHA256.
    LinearHmacSha256,
    /// PayPal transmission signature / cert verification.
    PayPalTransmission,
    /// PagerDuty shared secret / signature header.
    PagerDutySignature,
}

impl SignatureScheme {
    /// Stable id for KDL/JSON emission.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StripeSignature => "stripe-signature",
            Self::GitHubHmacSha256 => "github-hmac-sha256",
            Self::GitLabToken => "gitlab-token",
            Self::SlackSigningSecret => "slack-signing-secret",
            Self::DiscordEd25519 => "discord-ed25519",
            Self::ShopifyHmacSha256 => "shopify-hmac-sha256",
            Self::TwilioSignature => "twilio-signature",
            Self::StandardWebhooks => "standard-webhooks",
            Self::Svix => "svix",
            Self::HubSpotV3 => "hubspot-v3",
            Self::IpAllowlist => "ip-allowlist",
            Self::BitbucketHmacSha256 => "bitbucket-hmac-sha256",
            Self::VercelSignature => "vercel-signature",
            Self::LinearHmacSha256 => "linear-hmac-sha256",
            Self::PayPalTransmission => "paypal-transmission",
            Self::PagerDutySignature => "pagerduty-signature",
        }
    }

    /// Primary HTTP header carrying the signature or shared token, when known.
    #[must_use]
    pub const fn header(self) -> Option<&'static str> {
        match self {
            Self::StripeSignature => Some("Stripe-Signature"),
            Self::GitHubHmacSha256 => Some("X-Hub-Signature-256"),
            Self::GitLabToken => Some("X-Gitlab-Token"),
            Self::SlackSigningSecret => Some("X-Slack-Signature"),
            Self::DiscordEd25519 => Some("X-Signature-Ed25519"),
            Self::ShopifyHmacSha256 => Some("X-Shopify-Hmac-Sha256"),
            Self::TwilioSignature => Some("X-Twilio-Signature"),
            Self::StandardWebhooks | Self::Svix => Some("webhook-signature"),
            Self::HubSpotV3 => Some("X-HubSpot-Signature-v3"),
            Self::IpAllowlist => None,
            Self::BitbucketHmacSha256 => Some("X-Hub-Signature"),
            Self::VercelSignature => Some("x-vercel-signature"),
            Self::LinearHmacSha256 => Some("Linear-Signature"),
            Self::PayPalTransmission => Some("PAYPAL-TRANSMISSION-SIG"),
            Self::PagerDutySignature => Some("X-PagerDuty-Signature"),
        }
    }

    /// Wire encoding for the digest bytes, when applicable.
    #[must_use]
    pub const fn encoding(self) -> Option<SignatureEncoding> {
        match self {
            Self::StripeSignature
            | Self::GitHubHmacSha256
            | Self::SlackSigningSecret
            | Self::BitbucketHmacSha256
            | Self::VercelSignature
            | Self::LinearHmacSha256
            | Self::StandardWebhooks
            | Self::Svix
            | Self::DiscordEd25519 => Some(SignatureEncoding::Hex),
            Self::ShopifyHmacSha256 | Self::TwilioSignature => Some(SignatureEncoding::Base64),
            Self::GitLabToken
            | Self::HubSpotV3
            | Self::IpAllowlist
            | Self::PayPalTransmission
            | Self::PagerDutySignature => None,
        }
    }
}

/// How a signature digest is encoded on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureEncoding {
    /// Hexadecimal digest.
    Hex,
    /// Base64 digest.
    Base64,
}

impl SignatureEncoding {
    /// Stable id for emission.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hex => "hex",
            Self::Base64 => "base64",
        }
    }
}
