//! CI-friendly conformance runner.
//!
//! ```sh
//! cargo run --example conformance
//! HOOKS_CONFORMANCE_VERBOSE=1 cargo run --example conformance
//! ```
//!
//! Exit code 0 only when there are zero failures.
//! Skips mean “no assertable fixture for this layer yet” (or true N/A),
//! not hidden failures — see README.

use std::collections::BTreeMap;
use std::process::ExitCode;

use hooks::conformance::{run_suite, CheckOutcome};

fn main() -> ExitCode {
    let report = run_suite();
    println!("{}", report.summary());

    let verbose = std::env::var_os("HOOKS_CONFORMANCE_VERBOSE").is_some();
    let mut skip_reasons: BTreeMap<&str, usize> = BTreeMap::new();

    for check in report.checks() {
        if check.outcome != CheckOutcome::Skip {
            continue;
        }
        *skip_reasons.entry(check.detail.as_str()).or_insert(0) += 1;
        if verbose {
            println!(
                "  SKIP {}.{} — {}",
                check.provider.as_str(),
                check.check,
                check.detail
            );
        }
    }

    if !skip_reasons.is_empty() {
        println!("skip breakdown (why coverage is incomplete):");
        for (reason, count) in skip_reasons {
            println!("  {count:>3} × {reason}");
        }
        if !verbose {
            println!("re-run with HOOKS_CONFORMANCE_VERBOSE=1 to list each skip");
        }
    }

    if report.ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
