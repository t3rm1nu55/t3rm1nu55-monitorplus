//! t3rm-exporter — Prometheus `/metrics` endpoint binary.
//!
//! **Sprint 0 scaffold.** Real exporter lands in Sprint 2.
//!
//! Pair with Grafana, VictoriaMetrics, or any Prometheus-compatible scraper to get
//! dashboards and alerting on top of t3rm's deep Apple Silicon metrics.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("t3rm-exporter scaffold (Sprint 0)");
    eprintln!("t3rm-exporter: Sprint 0 scaffold. Real Prometheus exporter coming in Sprint 2.");
    Ok(())
}
