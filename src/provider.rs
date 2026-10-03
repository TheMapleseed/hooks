//! Catalog of supported HTTP webhook providers.

use crate::common::SignatureScheme;

/// A first-class HTTP webhook provider this crate can describe, sign, and (where
/// implemented) register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Provider {
    /// Stripe payment / billing webhooks.
    Stripe,
    /// GitHub repository webhooks.
    GitHub,
    /// GitLab project/group webhooks.
    GitLab,
    /// Bitbucket Cloud webhooks.
    Bitbucket,
    /// Slack Events API / interactive payloads.
    Slack,
    /// Discord interactions / outbound webhooks metadata.
    Discord,
    /// Shopify Admin webhooks.
    Shopify,
    /// Twilio SMS / voice status callbacks.
    Twilio,
    /// SendGrid Event Webhook.
    SendGrid,
    /// Linear issue webhooks.
    Linear,
    /// Clerk (Svix) webhooks.
    Clerk,
    /// Vercel deployment / log drain hooks.
    Vercel,
    /// HubSpot CRM webhooks.
    HubSpot,
    /// PayPal REST webhooks.
    PayPal,
    /// PagerDuty webhooks.
    PagerDuty,
    /// Standard Webhooks (Svix ecosystem: Resend, Polar, etc.).
    StandardWebhooks,
}

impl Provider {
    /// Stable lowercase id used in emitted documents.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stripe => "stripe",
            Self::GitHub => "github",
            Self::GitLab => "gitlab",
            Self::Bitbucket => "bitbucket",
            Self::Slack => "slack",
            Self::Discord => "discord",
            Self::Shopify => "shopify",
            Self::Twilio => "twilio",
            Self::SendGrid => "sendgrid",
            Self::Linear => "linear",
            Self::Clerk => "clerk",
            Self::Vercel => "vercel",
            Self::HubSpot => "hubspot",
            Self::PayPal => "paypal",
            Self::PagerDuty => "pagerduty",
            Self::StandardWebhooks => "standard-webhooks",
        }
    }

    /// Default signature scheme for this provider.
    #[must_use]
    pub const fn signature_scheme(self) -> SignatureScheme {
        match self {
            Self::Stripe => SignatureScheme::StripeSignature,
            Self::GitHub => SignatureScheme::GitHubHmacSha256,
            Self::GitLab => SignatureScheme::GitLabToken,
            Self::Bitbucket => SignatureScheme::BitbucketHmacSha256,
            Self::Slack => SignatureScheme::SlackSigningSecret,
            Self::Discord => SignatureScheme::DiscordEd25519,
            Self::Shopify => SignatureScheme::ShopifyHmacSha256,
            Self::Twilio => SignatureScheme::TwilioSignature,
            Self::SendGrid => SignatureScheme::IpAllowlist,
            Self::Linear => SignatureScheme::LinearHmacSha256,
            Self::Clerk => SignatureScheme::Svix,
            Self::Vercel => SignatureScheme::VercelSignature,
            Self::HubSpot => SignatureScheme::HubSpotV3,
            Self::PayPal => SignatureScheme::PayPalTransmission,
            Self::PagerDuty => SignatureScheme::PagerDutySignature,
            Self::StandardWebhooks => SignatureScheme::StandardWebhooks,
        }
    }

    /// Every provider variant, in stable display order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Stripe,
            Self::GitHub,
            Self::GitLab,
            Self::Bitbucket,
            Self::Slack,
            Self::Discord,
            Self::Shopify,
            Self::Twilio,
            Self::SendGrid,
            Self::Linear,
            Self::Clerk,
            Self::Vercel,
            Self::HubSpot,
            Self::PayPal,
            Self::PagerDuty,
            Self::StandardWebhooks,
        ]
    }
}
