//! API-standard conformance harness for webhook shapes.
//!
//! Validates that configs, signature headers, payloads, and registration
//! request shapes match each provider's documented wire format — so the build
//! fails when a provider surface drifts.

mod payload;
mod registration;
mod report;
mod signature;
mod standards;

use crate::provider::Provider;

pub use report::{CheckOutcome, CheckResult, ConformanceReport};
pub use standards::{
    all_standards, pattern_matches, standard_for, ProviderStandard, SignatureStandard,
};

use payload::check_payloads;
use registration::check_registration_shapes;
use report::ConformanceReport as Report;
use signature::check_signatures;
use standards::standard_for as lookup_standard;

/// Run the full conformance suite across every [`Provider`].
#[must_use]
pub fn run_suite() -> ConformanceReport {
    let mut report = Report::new();

    for provider in Provider::all() {
        let Some(standard) = lookup_standard(*provider) else {
            report.push(CheckResult::skip(
                *provider,
                "catalog",
                "no conformance standard registered",
            ));
            continue;
        };

        report.extend(check_provider_standard(*provider, &standard));
        report.extend(check_signatures(*provider, &standard));
        report.extend(check_payloads(*provider, &standard));
        report.extend(check_registration_shapes(*provider, &standard));
    }

    report
}

fn check_provider_standard(
    provider: Provider,
    standard: &ProviderStandard,
) -> Vec<CheckResult> {
    let mut out = Vec::new();

    let scheme = provider.signature_scheme();
    if scheme != standard.signature.scheme {
        out.push(CheckResult::fail(
            provider,
            "signature.scheme",
            format!(
                "Provider::signature_scheme()={scheme:?} != standard {:?}",
                standard.signature.scheme
            ),
        ));
    } else {
        out.push(CheckResult::pass(
            provider,
            "signature.scheme",
            "matches Provider::signature_scheme()",
        ));
    }

    if scheme.header() != standard.signature.primary_header {
        out.push(CheckResult::fail(
            provider,
            "signature.header",
            format!(
                "header {:?} != standard {:?}",
                scheme.header(),
                standard.signature.primary_header
            ),
        ));
    } else {
        out.push(CheckResult::pass(
            provider,
            "signature.header",
            "primary signature header matches standard",
        ));
    }

    out.extend(check_event_catalog(provider, standard));
    out
}

fn check_event_catalog(provider: Provider, standard: &ProviderStandard) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let events = event_wire_names(provider);

    if events.is_empty() {
        out.push(CheckResult::fail(
            provider,
            "events.catalog",
            "provider exposes zero typed events",
        ));
        return out;
    }

    let bad: Vec<_> = events
        .iter()
        .filter(|name| !standard.event_name_ok(name))
        .cloned()
        .collect();

    if bad.is_empty() {
        out.push(CheckResult::pass(
            provider,
            "events.catalog",
            format!("{} typed events match wire-name rules", events.len()),
        ));
    } else {
        out.push(CheckResult::fail(
            provider,
            "events.catalog",
            format!("events violate wire-name rules: {bad:?}"),
        ));
    }
    out
}

fn event_wire_names(provider: Provider) -> Vec<String> {
    match provider {
        Provider::Stripe => map_events(crate::stripe::Event::all()),
        Provider::GitHub => map_events(crate::github::Event::all()),
        Provider::GitLab => map_events(crate::gitlab::Event::all()),
        Provider::Bitbucket => map_events(crate::bitbucket::Event::all()),
        Provider::Slack => map_events(crate::slack::Event::all()),
        Provider::Discord => map_events(crate::discord::Event::all()),
        Provider::Shopify => map_events(crate::shopify::Event::all()),
        Provider::Twilio => map_events(crate::twilio::Event::all()),
        Provider::SendGrid => map_events(crate::sendgrid::Event::all()),
        Provider::Linear => map_events(crate::linear::Event::all()),
        Provider::Clerk => map_events(crate::clerk::Event::all()),
        Provider::Vercel => map_events(crate::vercel::Event::all()),
        Provider::HubSpot => map_events(crate::hubspot::Event::all()),
        Provider::PayPal => map_events(crate::paypal::Event::all()),
        Provider::PagerDuty => map_events(crate::pagerduty::Event::all()),
        Provider::StandardWebhooks => map_events(crate::standard_webhooks::Event::all()),
    }
}

fn map_events<E: Copy + std::fmt::Display>(events: &[E]) -> Vec<String> {
    events.iter().map(|e| e.to_string()).collect()
}

/// Panic with a readable summary when any conformance check failed.
pub fn assert_suite_passes(report: &ConformanceReport) {
    if report.ok() {
        return;
    }
    panic!(
        "webhook API conformance failed ({}/{} checks):\n{}",
        report.failed(),
        report.total(),
        report.summary()
    );
}
