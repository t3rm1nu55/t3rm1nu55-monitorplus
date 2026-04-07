//! t3rm-mcp — MCP server binary for t3rm1nu55-monitorplus.
//!
//! **Sprint 0 scaffold.** Real MCP server implementation lands in Sprint 2.
//!
//! Design goals (see `CLAUDE.md` design rule #2 and `PLAN.md` Sprint 2):
//!
//! - Tool responses target **<500 tokens per response** (Context Rot mitigation)
//! - Use the MCP Tasks primitive (2025-11-25 spec) for async historical range queries
//! - Four core tools: `system_snapshot`, `component_detail`, `process_top`, `anomaly_scan`
//! - Return pre-computed summaries (current, 1-min avg, 24h p95, trend, anomaly flag),
//!   never raw time series

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("t3rm-mcp scaffold (Sprint 0)");
    eprintln!("t3rm-mcp: Sprint 0 scaffold. Real MCP server coming in Sprint 2.");
    Ok(())
}
