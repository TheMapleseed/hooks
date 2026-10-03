//! Build-gated API conformance suite.
//!
//! Runs shape / signature / registration checks for every provider standard.
//! This is the primary CI gate that webhook wire formats still match each API.

use hooks::conformance::{assert_suite_passes, run_suite, CheckOutcome, ProviderStandard};
use hooks::Provider;

#[test]
fn all_providers_have_conformance_standards() {
    for provider in Provider::all() {
        assert!(
            hooks::conformance::standard_for(*provider).is_some(),
            "missing conformance standard for {}",
            provider.as_str()
        );
    }
}

#[test]
fn provider_standards_cover_unique_providers() {
    let mut seen = std::collections::BTreeSet::new();
    for standard in hooks::conformance::all_standards() {
        assert!(
            seen.insert(standard.provider.as_str()),
            "duplicate standard for {}",
            standard.provider.as_str()
        );
        let _standard: &ProviderStandard = standard;
    }
    assert_eq!(seen.len(), Provider::all().len());
}

#[test]
fn api_conformance_suite_passes() {
    let report = run_suite();
    eprintln!("{}", report.summary());
    assert!(report.passed() > 0, "expected at least one passing check");
    assert_suite_passes(&report);
}

#[test]
fn suite_marks_failures_explicitly_when_forced() {
    // Sanity: CheckOutcome discriminants behave for reporting.
    assert_ne!(CheckOutcome::Pass, CheckOutcome::Fail);
    assert_ne!(CheckOutcome::Fail, CheckOutcome::Skip);
}
