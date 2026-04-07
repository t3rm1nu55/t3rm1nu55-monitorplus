//! t3rm-core — collector library for t3rm1nu55-monitorplus.
//!
//! See the `CLAUDE.md` and `PLAN.md` at the repo root for architecture and design rules.
//! Current status: **Sprint 0 scaffold**. Public surface is stub APIs only.
//!
//! # Privilege tiers
//!
//! Every metric source declares which privilege tier it requires via [`Tier`]:
//!
//! - **Tier 1** — no sudo, no entitlements. IOReport, SMC reads, sysinfo, libproc-self.
//! - **Tier 2** — root. kperf PMU counters via privileged sidecar.
//! - **Tier 3** — Apple-approved entitlement. Endpoint Security framework events.
//!
//! Cargo features gate which tiers are compiled in (`tier1`, `tier2`, `tier3`).
//! The CI matrix builds all three release variants on every commit.

#![forbid(unsafe_code)] // will be relaxed to deny(unsafe_op_in_unsafe_fn) when FFI lands in Sprint 1
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

/// Privilege level required by a metric source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    /// No sudo, no entitlements. IOReport, SMC reads, sysinfo, libproc-self.
    One,
    /// Root required. kperf PMU counters via privileged sidecar.
    Two,
    /// Apple-approved entitlement required. Endpoint Security framework.
    Three,
}

/// A metric source. Sampled periodically by the collector loop.
///
/// Sprint 1 will add concrete implementations for IOReport, SMC, sysinfo, etc.
pub trait Source {
    /// Human-readable name, used in logs and MCP tool descriptions.
    fn name(&self) -> &'static str;

    /// Privilege tier required to use this source.
    fn tier(&self) -> Tier;
}

/// Tier 2: kperf PMU counter access. Real implementation lands in Sprint 3.
#[cfg(feature = "__kperf")]
pub mod kperf {}

/// Tier 3: Endpoint Security framework. Real implementation lands in Sprint 7.
#[cfg(feature = "__es")]
pub mod endpoint_security {}

/// Research-track features: per-process power inference, macOS-PSI, histogram compression.
/// Real implementation lands in Sprint 4.
#[cfg(feature = "research")]
pub mod research {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_equality() {
        assert_eq!(Tier::One, Tier::One);
        assert_ne!(Tier::One, Tier::Two);
    }

    #[test]
    fn tier_is_copy() {
        let a = Tier::Two;
        let b = a;
        assert_eq!(a, b);
    }
}
