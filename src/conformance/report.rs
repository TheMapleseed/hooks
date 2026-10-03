//! Conformance check results and reporting.

use crate::provider::Provider;

/// Outcome of a single conformance check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckOutcome {
    /// Check succeeded.
    Pass,
    /// Check failed — shape does not meet the API standard.
    Fail,
    /// Check intentionally skipped (unsupported / N/A).
    Skip,
}

/// One named check against a provider standard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    /// Provider under test.
    pub provider: Provider,
    /// Stable check id (`signature.header`, `payload.required_keys`, …).
    pub check: String,
    /// Outcome.
    pub outcome: CheckOutcome,
    /// Human-readable detail.
    pub detail: String,
}

impl CheckResult {
    /// Construct a passing check.
    pub fn pass(provider: Provider, check: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            provider,
            check: check.into(),
            outcome: CheckOutcome::Pass,
            detail: detail.into(),
        }
    }

    /// Construct a failing check.
    pub fn fail(provider: Provider, check: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            provider,
            check: check.into(),
            outcome: CheckOutcome::Fail,
            detail: detail.into(),
        }
    }

    /// Construct a skipped check.
    pub fn skip(provider: Provider, check: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            provider,
            check: check.into(),
            outcome: CheckOutcome::Skip,
            detail: detail.into(),
        }
    }
}

/// Aggregated suite results.
#[derive(Debug, Clone, Default)]
pub struct ConformanceReport {
    checks: Vec<CheckResult>,
}

impl ConformanceReport {
    /// Empty report.
    #[must_use]
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    /// Append one check.
    pub fn push(&mut self, result: CheckResult) {
        self.checks.push(result);
    }

    /// Append many checks.
    pub fn extend(&mut self, results: impl IntoIterator<Item = CheckResult>) {
        self.checks.extend(results);
    }

    /// All checks.
    #[must_use]
    pub fn checks(&self) -> &[CheckResult] {
        &self.checks
    }

    /// Total checks run (including skips).
    #[must_use]
    pub fn total(&self) -> usize {
        self.checks.len()
    }

    /// Number of failures.
    #[must_use]
    pub fn failed(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.outcome == CheckOutcome::Fail)
            .count()
    }

    /// Number of passes.
    #[must_use]
    pub fn passed(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.outcome == CheckOutcome::Pass)
            .count()
    }

    /// Number of skips.
    #[must_use]
    pub fn skipped(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.outcome == CheckOutcome::Skip)
            .count()
    }

    /// Whether the suite is green (zero failures).
    #[must_use]
    pub fn ok(&self) -> bool {
        self.failed() == 0
    }

    /// Multi-line summary for logs / CI.
    #[must_use]
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "conformance: {} passed, {} failed, {} skipped ({} total)",
            self.passed(),
            self.failed(),
            self.skipped(),
            self.total()
        ));
        for check in &self.checks {
            if check.outcome == CheckOutcome::Fail {
                lines.push(format!(
                    "  FAIL {}.{} — {}",
                    check.provider.as_str(),
                    check.check,
                    check.detail
                ));
            }
        }
        lines.join("\n")
    }
}
