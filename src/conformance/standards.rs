//! Declarative per-provider wire standards used by the conformance harness.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::common::SignatureScheme;
use crate::provider::Provider;

/// Signature header / format expectations for a provider.
#[derive(Debug, Clone)]
pub struct SignatureStandard {
    /// Scheme the provider must use.
    pub scheme: SignatureScheme,
    /// Primary authenticity header, if any.
    pub primary_header: Option<&'static str>,
    /// Regex the primary header value must match after signing (when signable).
    pub value_pattern: Option<&'static str>,
    /// Whether this crate can produce a signed header today.
    pub signable: bool,
    /// Extra required headers produced alongside the signature.
    pub required_headers: &'static [&'static str],
}

/// Full conformance standard for one provider.
#[derive(Debug, Clone)]
pub struct ProviderStandard {
    /// Provider this standard describes.
    pub provider: Provider,
    /// Signature rules.
    pub signature: SignatureStandard,
    /// Regex every typed event wire-name must match.
    pub event_pattern: &'static str,
    /// Required top-level JSON keys for synthetic outbound envelopes (if any).
    pub payload_required_keys: &'static [&'static str],
    /// When set, `object` field must equal this string (Stripe).
    pub payload_object: Option<&'static str>,
    /// Whether registration request shape is checked.
    pub registration_checked: bool,
}

impl ProviderStandard {
    /// Return true when `event` matches this provider's wire-name rules.
    #[must_use]
    pub fn event_name_ok(&self, event: &str) -> bool {
        pattern_matches(self.event_pattern, event)
    }
}

/// Anchored regex match used by conformance checks.
#[must_use]
pub fn pattern_matches(pattern: &str, text: &str) -> bool {
    static CACHE: OnceLock<Mutex<HashMap<String, regex::Regex>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().expect("regex cache");
    let re = map.entry(pattern.to_owned()).or_insert_with(|| {
        regex::Regex::new(&format!("^(?:{pattern})$")).unwrap_or_else(|err| {
            panic!("invalid conformance regex `{pattern}`: {err}");
        })
    });
    re.is_match(text)
}

/// Look up the standard for `provider`, if one is defined.
#[must_use]
pub fn standard_for(provider: Provider) -> Option<ProviderStandard> {
    STANDARDS.iter().find(|s| s.provider == provider).cloned()
}

/// All registered standards.
#[must_use]
pub fn all_standards() -> &'static [ProviderStandard] {
    STANDARDS
}

const STANDARDS: &[ProviderStandard] = &[
    ProviderStandard {
        provider: Provider::Stripe,
        signature: SignatureStandard {
            scheme: SignatureScheme::StripeSignature,
            primary_header: Some("Stripe-Signature"),
            value_pattern: Some(r"t=[0-9]+,v1=[0-9a-f]{64}"),
            signable: true,
            required_headers: &["Content-Type"],
        },
        event_pattern: r"[a-z][a-z0-9_]*(\.[a-z0-9_]+)+",
        payload_required_keys: &["id", "object", "type", "data", "created", "livemode"],
        payload_object: Some("event"),
        registration_checked: true,
    },
    ProviderStandard {
        provider: Provider::GitHub,
        signature: SignatureStandard {
            scheme: SignatureScheme::GitHubHmacSha256,
            primary_header: Some("X-Hub-Signature-256"),
            value_pattern: Some(r"sha256=[0-9a-f]{64}"),
            signable: true,
            required_headers: &["Content-Type"],
        },
        event_pattern: r"[a-z][a-z0-9_]*",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: true,
    },
    ProviderStandard {
        provider: Provider::GitLab,
        signature: SignatureStandard {
            scheme: SignatureScheme::GitLabToken,
            primary_header: Some("X-Gitlab-Token"),
            value_pattern: Some(r".+"),
            signable: true,
            required_headers: &["Content-Type"],
        },
        event_pattern: r"[A-Za-z][A-Za-z0-9 ]* Hook",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::Bitbucket,
        signature: SignatureStandard {
            scheme: SignatureScheme::BitbucketHmacSha256,
            primary_header: Some("X-Hub-Signature"),
            value_pattern: Some(r"sha256=[0-9a-f]{64}"),
            signable: true,
            required_headers: &["Content-Type"],
        },
        event_pattern: r"[a-z]+:[a-z_]+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::Slack,
        signature: SignatureStandard {
            scheme: SignatureScheme::SlackSigningSecret,
            primary_header: Some("X-Slack-Signature"),
            value_pattern: Some(r"v0=[0-9a-f]{64}"),
            signable: true,
            required_headers: &["X-Slack-Request-Timestamp", "Content-Type"],
        },
        event_pattern: r"[a-z][a-z0-9_]*",
        payload_required_keys: &["type", "team_id", "event", "event_id", "event_time"],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::Discord,
        signature: SignatureStandard {
            scheme: SignatureScheme::DiscordEd25519,
            primary_header: Some("X-Signature-Ed25519"),
            value_pattern: None,
            signable: false,
            required_headers: &[],
        },
        event_pattern: r"[A-Z][A-Z0-9_]*|interaction\.[a-z_]+|channel\.webhook\.execute",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::Shopify,
        signature: SignatureStandard {
            scheme: SignatureScheme::ShopifyHmacSha256,
            primary_header: Some("X-Shopify-Hmac-Sha256"),
            value_pattern: Some(r"[A-Za-z0-9+/]+=*"),
            signable: true,
            required_headers: &["Content-Type"],
        },
        event_pattern: r"[a-z_]+/[a-z_]+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::Twilio,
        signature: SignatureStandard {
            scheme: SignatureScheme::TwilioSignature,
            primary_header: Some("X-Twilio-Signature"),
            value_pattern: Some(r"[A-Za-z0-9+/]+=*"),
            signable: true,
            required_headers: &["Content-Type"],
        },
        event_pattern: r"[a-z]+\.[a-z_]+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::SendGrid,
        signature: SignatureStandard {
            scheme: SignatureScheme::IpAllowlist,
            primary_header: None,
            value_pattern: None,
            signable: false,
            required_headers: &[],
        },
        event_pattern: r"[a-z_]+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::Linear,
        signature: SignatureStandard {
            scheme: SignatureScheme::LinearHmacSha256,
            primary_header: Some("Linear-Signature"),
            value_pattern: Some(r"[0-9a-f]{64}"),
            signable: true,
            required_headers: &["Content-Type"],
        },
        event_pattern: r"[A-Z][A-Za-z]+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::Clerk,
        signature: SignatureStandard {
            scheme: SignatureScheme::Svix,
            primary_header: Some("webhook-signature"),
            value_pattern: Some(r"v1,[A-Za-z0-9+/=]+"),
            signable: true,
            required_headers: &["webhook-id", "webhook-timestamp", "Content-Type"],
        },
        event_pattern: r"[a-zA-Z]+\.[a-z]+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::Vercel,
        signature: SignatureStandard {
            scheme: SignatureScheme::VercelSignature,
            primary_header: Some("x-vercel-signature"),
            value_pattern: Some(r"[0-9a-f]{64}"),
            signable: true,
            required_headers: &["Content-Type"],
        },
        event_pattern: r"[a-z][a-z0-9.-]*",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::HubSpot,
        signature: SignatureStandard {
            scheme: SignatureScheme::HubSpotV3,
            primary_header: Some("X-HubSpot-Signature-v3"),
            value_pattern: Some(r"[A-Za-z0-9+/]+=*"),
            signable: true,
            required_headers: &["X-HubSpot-Request-Timestamp", "Content-Type"],
        },
        event_pattern: r"[a-z]+\.[a-zA-Z]+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::PayPal,
        signature: SignatureStandard {
            scheme: SignatureScheme::PayPalTransmission,
            primary_header: Some("PAYPAL-TRANSMISSION-SIG"),
            value_pattern: None,
            signable: false,
            required_headers: &[],
        },
        event_pattern: r"[A-Z]+(\.[A-Z]+)+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::PagerDuty,
        signature: SignatureStandard {
            scheme: SignatureScheme::PagerDutySignature,
            primary_header: Some("X-PagerDuty-Signature"),
            value_pattern: None,
            signable: false,
            required_headers: &[],
        },
        event_pattern: r"[a-z]+\.[a-z_]+",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
    ProviderStandard {
        provider: Provider::StandardWebhooks,
        signature: SignatureStandard {
            scheme: SignatureScheme::StandardWebhooks,
            primary_header: Some("webhook-signature"),
            value_pattern: Some(r"v1,[A-Za-z0-9+/=]+"),
            signable: true,
            required_headers: &["webhook-id", "webhook-timestamp", "Content-Type"],
        },
        event_pattern: r"[a-z]+(\.[a-z_]+)?",
        payload_required_keys: &[],
        payload_object: None,
        registration_checked: false,
    },
];
