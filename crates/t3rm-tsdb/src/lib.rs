//! t3rm-tsdb — embedded time-series store for t3rm1nu55-monitorplus.
//!
//! **Sprint 0 scaffold.** Real TSDB implementation lands in Sprint 2.
//!
//! Planned architecture (see `PLAN.md` Sprint 2, tasks S2-1..S2-4):
//!
//! - **Gorilla encoding** (delta-of-delta timestamps + XOR float values) for ~10× compression.
//! - **`redb` backend** with 2-hour chunk boundaries matching Prometheus convention.
//! - **Last Value Cache** — in-memory `HashMap<SeriesKey, LatestSample>` served without
//!   disk I/O for MCP snapshot queries.
//! - **24-hour retention** by default, time-based expiry.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

/// Scaffold version marker. Will be removed when the real implementation lands.
pub const TSDB_VERSION: &str = "0.0.0-scaffold";

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        assert_eq!(super::TSDB_VERSION, "0.0.0-scaffold");
    }
}
