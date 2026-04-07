//! t3rm-pmud — privileged kperf sidecar for t3rm1nu55-monitorplus.
//!
//! **Sprint 0 scaffold.** Real kperf access is implemented in Sprint 3.
//!
//! When implemented, this binary will require root privileges to call
//! `kpc_force_all_ctrs_set(1)` and configure the kperf hardware counter framework.
//! The main `t3rm` daemon (unprivileged) will connect over a unix socket and read
//! counter samples. See `PLAN.md` Sprint 3 for the implementation plan.

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("t3rm-pmud scaffold (Sprint 0)");
    eprintln!(
        "t3rm-pmud: Sprint 0 scaffold. Real kperf sidecar coming in Sprint 3. \
         Will require root when implemented."
    );
    Ok(())
}
