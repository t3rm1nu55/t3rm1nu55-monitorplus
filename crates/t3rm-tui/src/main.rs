//! t3rm-tui — terminal dashboard binary.
//!
//! **Sprint 0 scaffold.** Real ratatui dashboard lands in Sprint 2.
//!
//! Planned layout: 4 panes (CPU+thermals, GPU+ANE power, Memory+swap, Top processes by watts).

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("t3rm-tui scaffold (Sprint 0)");
    eprintln!("t3rm-tui: Sprint 0 scaffold. Real dashboard coming in Sprint 2.");
    Ok(())
}
