//! Signature header shape checks against provider standards.

use crate::common::WebhookBuilder;
use crate::provider::Provider;
use crate::secret::Secret;
use crate::sign::{sign, SignRequest};

use super::report::CheckResult;
use super::standards::{pattern_matches, ProviderStandard};

/// Check signature metadata on a sample webhook config + produced headers.
pub(super) fn check_signatures(
    provider: Provider,
    standard: &ProviderStandard,
) -> Vec<CheckResult> {
    let mut out = Vec::new();

    match sample_webhook(provider) {
        Ok(webhook) => {
            if webhook.signature_scheme() == standard.signature.scheme.as_str() {
                out.push(CheckResult::pass(
                    provider,
                    "config.signature_scheme",
                    "webhook config advertises standard scheme",
                ));
            } else {
                out.push(CheckResult::fail(
                    provider,
                    "config.signature_scheme",
                    format!(
                        "config has {} want {}",
                        webhook.signature_scheme(),
                        standard.signature.scheme.as_str()
                    ),
                ));
            }
        }
        Err(err) => {
            out.push(CheckResult::fail(
                provider,
                "config.build",
                format!("could not build sample webhook: {err}"),
            ));
            return out;
        }
    }

    if !standard.signature.signable {
        out.push(CheckResult::skip(
            provider,
            "signature.produce",
            "signing not implemented / not applicable for this provider",
        ));
        return out;
    }

    let secret = match provider {
        Provider::StandardWebhooks | Provider::Clerk => Secret::new("whsec_c2VjcmV0"),
        _ => Secret::new("conformance_test_secret"),
    };

    let body = br#"{"conformance":true}"#;
    let signed = sign(SignRequest {
        scheme: standard.signature.scheme,
        body,
        secret: &secret,
        timestamp: 1_700_000_000,
        message_id: Some("msg_conformance_1"),
        destination_url: Some("https://conformance.example/hooks/receiver"),
    });

    let signed = match signed {
        Ok(s) => s,
        Err(err) => {
            out.push(CheckResult::fail(
                provider,
                "signature.produce",
                format!("sign() failed: {err}"),
            ));
            return out;
        }
    };

    if let Some(header_name) = standard.signature.primary_header {
        match signed.headers.get(header_name) {
            Some(value) => {
                if let Some(pattern) = standard.signature.value_pattern {
                    if pattern_matches(pattern, value) {
                        out.push(CheckResult::pass(
                            provider,
                            "signature.value_shape",
                            format!("{header_name} matches standard pattern"),
                        ));
                    } else {
                        out.push(CheckResult::fail(
                            provider,
                            "signature.value_shape",
                            format!("{header_name}={value:?} does not match /{pattern}/"),
                        ));
                    }
                } else {
                    out.push(CheckResult::pass(
                        provider,
                        "signature.value_shape",
                        format!("{header_name} present"),
                    ));
                }
            }
            None => out.push(CheckResult::fail(
                provider,
                "signature.value_shape",
                format!("missing required header {header_name}"),
            )),
        }
    }

    for required in standard.signature.required_headers {
        if signed.headers.contains_key(*required) {
            out.push(CheckResult::pass(
                provider,
                format!("signature.header.{required}"),
                "present",
            ));
        } else {
            out.push(CheckResult::fail(
                provider,
                format!("signature.header.{required}"),
                "missing required companion header",
            ));
        }
    }

    out
}

fn sample_webhook(provider: Provider) -> crate::error::Result<crate::common::Webhook> {
    WebhookBuilder::new(provider)
        .url("https://conformance.example/hooks/receiver")
        .event(first_event(provider))
        .secret_env("CONFORMANCE_SECRET")
        .build()
}

fn first_event(provider: Provider) -> &'static str {
    match provider {
        Provider::Stripe => crate::stripe::Event::PaymentIntentSucceeded.as_str(),
        Provider::GitHub => crate::github::Event::Push.as_str(),
        Provider::GitLab => crate::gitlab::Event::Push.as_str(),
        Provider::Bitbucket => crate::bitbucket::Event::RepoPush.as_str(),
        Provider::Slack => crate::slack::Event::Message.as_str(),
        Provider::Discord => crate::discord::Event::InteractionPing.as_str(),
        Provider::Shopify => crate::shopify::Event::OrdersCreate.as_str(),
        Provider::Twilio => crate::twilio::Event::MessagingInbound.as_str(),
        Provider::SendGrid => crate::sendgrid::Event::Delivered.as_str(),
        Provider::Linear => crate::linear::Event::Issue.as_str(),
        Provider::Clerk => crate::clerk::Event::UserCreated.as_str(),
        Provider::Vercel => crate::vercel::Event::DeploymentCreated.as_str(),
        Provider::HubSpot => crate::hubspot::Event::ContactCreation.as_str(),
        Provider::PayPal => crate::paypal::Event::PaymentCaptureCompleted.as_str(),
        Provider::PagerDuty => crate::pagerduty::Event::IncidentTriggered.as_str(),
        Provider::StandardWebhooks => crate::standard_webhooks::Event::MessageDelivered.as_str(),
    }
}
