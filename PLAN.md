# PLAN — t3rm1nu55-monitorplus

> Living document. Update with every meaningful task completion or decision change.
> See `CLAUDE.md` for stable architecture and design rules.

**Last updated:** 2026-04-07
**Current sprint:** Sprint 0 — Scaffold (executing)

---

## Sprint 0 — Scaffold

**Goal:** Stand up the workspace, both repos, the CI matrix, and the daily research cron. No collector implementation yet — just structure that compiles clean and locks in the architectural decisions before drift can happen.

### Tasks

- [x] **S0-1.** Write `CLAUDE.md` (project context) and `PLAN.md` (this file) — 2026-04-06
- [x] **S0-2.** Write `README.md`, `CONTRIBUTING.md`, `LICENSE-MIT`, `LICENSE-APACHE`, `.gitignore`, `rust-toolchain.toml` — 2026-04-07
- [x] **S0-3.** Write workspace `Cargo.toml` with shared deps and member declarations — 2026-04-07
- [x] **S0-4.** Scaffold 6 crate skeletons in `crates/` (compile clean, stub APIs only) — 2026-04-07
- [x] **S0-5.** Define Cargo features on `t3rm-core`: `tier1`, `tier2`, `tier3`, `research`, plus private `__kperf`/`__es` gates — 2026-04-07
- [x] **S0-6.** `.github/workflows/ci.yml` with three-variant build matrix (a-personal, b-pro, c-public) — 2026-04-07
- [ ] **S0-7.** Local `cargo check` to verify workspace compiles clean across all three variants
- [ ] **S0-8.** `git init`, initial commit (conventional commit message)
- [ ] **S0-9.** `gh repo create t3rm1nu55-monitorplus --public --source=. --push`
- [ ] **S0-10.** Verify CI green on first push (all 3 variants build)
- [ ] **S0-11.** Create `t3rm1nu55-ane-research` repo with journal/references/structure
- [ ] **S0-12.** Seed `references.md` from the research synthesis (Sonnet agent output)
- [ ] **S0-13.** Set up daily RemoteTrigger Sonnet agent for research stream
- [ ] **S0-14.** Verify RemoteTrigger fires once and updates the journal correctly

### Definition of done
- `cargo build --workspace` succeeds for all three variants on macos-latest CI
- Both repos exist on `github.com/t3rm1nu55`
- Daily RemoteTrigger registered and verified to run
- This `PLAN.md` updated with checkboxes ticked

---

## Sprint 1 — First working collector

**Goal:** A `t3rm` binary that prints CPU/mem/power/thermal samples to stdout, sourced from real IOReport + sysinfo + SMC. No TUI yet, no MCP yet — just prove the collector loop works against real Apple Silicon hardware.

### Tasks
- [ ] **S1-1.** Define `t3rm-core::sources::Source` trait (sample → `SourceData`, name, tier)
- [ ] **S1-2.** IOReport FFI module: `dlopen` libIOReport, declare extern "C" sigs, channel enumeration
- [ ] **S1-3.** Channel discovery layer: enumerate at startup, log diff from expected set, store availability map
- [ ] **S1-4.** Energy Model channel reader (per-cluster CPU/GPU/ANE power in mW)
- [ ] **S1-5.** CPU Stats channel reader (P/E core frequency residency)
- [ ] **S1-6.** sysinfo wrapper for CPU%/mem/disk/net/proc breadth
- [ ] **S1-7.** SMC HID hub reader for temps + fans (Apple Silicon path, not legacy IOKit)
- [ ] **S1-8.** CLI binary `t3rm-cli` that pretty-prints one sample per second to stdout
- [ ] **S1-9.** Test on author's hardware; document which IOReport channels are present
- [ ] **S1-10.** Multi-cluster (M-Pro/Max/Ultra) topology detection — stub if no test hardware
- [ ] **S1-11.** README screenshot/asciinema of `t3rm-cli` running

### Definition of done
- `cargo run -p t3rm-cli` prints CPU power, GPU power, P/E frequencies, RAM, swap, top 5 procs by CPU once per second
- Channel discovery layer logs available channels on startup
- Test machine's chip topology is correctly detected

---

## Sprint 2 — Storage + transports

**Goal:** Add the Gorilla TSDB and stand up the MCP server, Prometheus exporter, and ratatui TUI in parallel — all reading from the same `t3rm-core` collector.

### Tasks
- [ ] **S2-1.** `t3rm-tsdb`: Gorilla encoding (delta-of-delta timestamps + XOR float values)
- [ ] **S2-2.** `t3rm-tsdb`: redb backend with 2-hour chunk boundaries
- [ ] **S2-3.** `t3rm-tsdb`: Last Value Cache (in-memory `HashMap<SeriesKey, LatestSample>`)
- [ ] **S2-4.** `t3rm-tsdb`: Time-based expiry (24-hour ring buffer default)
- [ ] **S2-5.** `t3rm-mcp`: MCP server skeleton speaking 2025-11-25 spec
- [ ] **S2-6.** `t3rm-mcp`: tool `system_snapshot` (no args, <200 tokens, all subsystems at current second)
- [ ] **S2-7.** `t3rm-mcp`: tool `component_detail(component, window_minutes)`
- [ ] **S2-8.** `t3rm-mcp`: tool `process_top(metric, n, window_minutes)`
- [ ] **S2-9.** `t3rm-mcp`: tool `anomaly_scan()` returning only series outside their 1h baseline
- [ ] **S2-10.** `t3rm-mcp`: Tasks primitive for async historical range queries (>100ms)
- [ ] **S2-11.** `t3rm-exporter`: Prometheus `/metrics` endpoint via `metrics` + `metrics-exporter-prometheus`
- [ ] **S2-12.** `t3rm-tui`: ratatui dashboard with 4 panes (CPU, GPU, Memory, Top Procs)
- [ ] **S2-13.** Wire all four binaries to the same `t3rm-core` via shared in-process collector

### Definition of done
- `t3rm-tui` shows live dashboard refreshing at 1 Hz
- `t3rm-mcp` registered with Claude Desktop locally; LLM can answer "what's using my CPU"
- `t3rm-exporter` scraped by a local Prometheus, viewable in Grafana
- All MCP tool responses verified <500 tokens via test fixtures

---

## Sprint 3 — Tier 2 (kperf PMU + privileged sidecar)

**Goal:** Add kperf PMU counters via the privileged sidecar pattern. Begin collecting paired (PMU vector, IOReport energy delta) training data for Sprint 4's per-process power regression.

### Tasks
- [ ] **S3-1.** `t3rm-pmud`: setuid sidecar binary, kperf framework loaded via `dlopen`
- [ ] **S3-2.** Configure 8 programmable counters from chip-specific plist database
- [ ] **S3-3.** `kpc_force_all_ctrs_set(1)` + Instruments.app lock detection
- [ ] **S3-4.** M4 early-revision `kpc_set_config` failure probe + fallback path
- [ ] **S3-5.** Unix socket protocol between `t3rm` and `t3rm-pmud` (length-prefixed msgpack)
- [ ] **S3-6.** Sample loop: every 1s, read counters, send to main daemon
- [ ] **S3-7.** Per-thread (`kpc_get_thread_counters`) and per-CPU (`kpc_get_cpu_counters`) modes
- [ ] **S3-8.** Training data collector mode: write paired `(timestamp, PMU vector, IOReport energy delta, process attribution)` rows to disk
- [ ] **S3-9.** Documentation: how to run with sudo safely, what root sees vs. what main daemon sees
- [ ] **S3-10.** Variant B/C CI: ensure C build excludes the sidecar entirely

### Definition of done
- `sudo t3rm-pmud --serve` runs as a daemon, `t3rm` connects and shows IPC + cache miss rate per core
- 1 hour of paired training data collected during normal use
- Variant C still builds without `t3rm-pmud` in the build graph

---

## Sprint 4 — Research features (the differentiators)

**Goal:** Build per-process power inference and the macOS PSI equivalent. These are the genuinely-novel personal-tool capabilities — the reason this project exists vs. just using macmon.

### Tasks
- [ ] **S4-1.** Train per-process power model: load Sprint 3 paired data, fit linear/tree regression in Rust (`linfa` or `smartcore`)
- [ ] **S4-2.** Validate on held-out workloads, document RMSE
- [ ] **S4-3.** Power inference layer: at sample time, multiply each process's PMU vector by the model → estimated watts per process
- [ ] **S4-4.** macOS PSI: combine kperf thread stall counters + sysctl memory pressure + IOReport thermal throttle into unified `cpu_pressure`, `mem_pressure`, `io_pressure`, `thermal_pressure` (0–100 scale matching Linux PSI semantics)
- [ ] **S4-5.** Multi-cluster (M-Pro/Max/Ultra) IOReport attribution fix — only if test hardware available
- [ ] **S4-6.** Histogram-aware compression for DVFS frequency residency (`t3rm-tsdb` enhancement)
- [ ] **S4-7.** Expose per-process watts and pressure metrics via MCP `anomaly_scan` tool
- [ ] **S4-8.** TUI: add per-process watts column to top processes pane
- [ ] **S4-9.** Write up the regression methodology for the research repo

### Definition of done
- `t3rm` shows per-process watts in addition to per-process CPU%
- All four pressure metrics exposed and visible in TUI/MCP/exporter
- Methodology write-up published in `t3rm1nu55-ane-research/findings/`

---

## Sprint 5 — Polish + first public release (variant C)

**Goal:** Ship `t3rm-public` to a public Homebrew formula. This is the "gives back to community" milestone.

### Tasks
- [ ] **S5-1.** Variant C feature audit: ensure no Tier 2/3 leaks
- [ ] **S5-2.** Homebrew formula written and tested
- [ ] **S5-3.** GitHub release with binary artifacts (universal binary if possible)
- [ ] **S5-4.** README polish: features matrix per variant, install instructions, screenshots
- [ ] **S5-5.** CHANGELOG.md
- [ ] **S5-6.** Submit to `homebrew-core` or maintain own tap
- [ ] **S5-7.** Announce on HN, /r/MacOS, lobste.rs

### Definition of done
- `brew install t3rm-public` works for any user
- First external user reports running it successfully

---

## Future sprints (sketch)

- **Sprint 6:** Variant B (`t3rm-pro`) — Apple Developer ID signing, Homebrew tap, kperf sidecar packaging
- **Sprint 7:** Variant A (`t3rm`) — Endpoint Security entitlement application, file/process/network flow events
- **Sprint 8:** MCP diagnostic accuracy benchmark — measure LLM diagnosis accuracy with real fault injection
- **Sprint 9:** Histogram compression validation, TSDB performance tuning
- **Sprint 10:** Alternative storage backends (write-through to remote Prometheus, OpenTelemetry OTLP)

---

## Decisions log

> Append-only. Always include date and rationale.

- **2026-04-06.** Project framing: "high-ambition personal tool," not pure product. Per-process power and macOS PSI are v1 features, not deferred research. *Rationale: user explicitly chose research-flavored option after seeing the cuts list; values capability over time-to-first-release.*

- **2026-04-06.** Three release variants from day one with CI matrix. *Rationale: feature flag drift is painful to retrofit; locking in the architecture on day one with a CI matrix that enforces it costs ~150 lines and pays back forever.*

- **2026-04-06.** Separate `t3rm1nu55-ane-research` sister repo with daily RemoteTrigger Sonnet cron. *Rationale: AMX/ANE counter exposure is an active reverse-engineering frontier; user wants passive intelligence stream without polluting main repo. Daily cadence (not weekly) per user preference.*

- **2026-04-06.** No vendored reverse-engineered headers. We declare extern "C" function signatures in our own code, load private frameworks via `dlopen` at runtime. *Rationale: copying headers from `IOReport_decompile`-style projects creates IP exposure; the dlopen + manual sigs pattern is what macmon, asitop, and socpowerbud all use without legal trouble.*

- **2026-04-06.** Per-process network bandwidth OUT of scope for v1. *Rationale: NetworkStatistics.framework's kernel socket disappeared in macOS Big Sur with no replacement; the only paths are Apple-entitlement-gated (ES), root + slow (proc_pidfdinfo polling), or fragile (nettop wrapping). Document as known gap, revisit when ES entitlement obtained.*

- **2026-04-06.** AMX/ANE throughput counter exposure is research-track only, NOT in main repo v1. Power-on/off state via IOReport Energy Model is in scope. *Rationale: user wants the AMX/ANE work pursued via the sister research repo, not embedded in main project where it would slow down sprint cadence.*

- **2026-04-06.** Use the `iSMC` HID sensor hub approach for SMC reads, NOT the legacy Rust `smc` crate. *Rationale: sensors moved to a HID sensor hub on M-series Macs; the legacy crate may return no data or wrong data on Apple Silicon.*

- **2026-04-06.** TSDB stack: Gorilla encoding (delta-of-delta + XOR floats) on top of `redb`, with a separate in-memory Last Value Cache for snapshot queries. *Rationale: Gorilla is the proven baseline for monitoring data (10× compression, ~51% of samples to 1 bit when stable); redb is pure-Rust with no system deps, ACID, and outperforms LMDB on individual writes; LVC pattern from InfluxDB 3 keeps MCP snapshot queries off the disk.*

---

## Open questions

> Things we don't know yet that will affect future sprints. Capture here so they're not forgotten.

- **Test hardware.** What chip(s) does the author have for testing? Affects multi-cluster work in Sprint 1/4.
- **Apple Developer ID.** When does the author want to apply, and what entitlements should we request first? Affects Sprint 6.
- **ES entitlement application.** Submit when? Apple's review process is multi-week. Affects Sprint 7.
- **MCP Rust SDK readiness.** When does an SDK speaking the 2025-11-25 spec ship? Until then, we target the spec directly.
- **Histogram compression scheme.** Is delta-encoding histogram bins the right approach, or is there a better technique? Open design problem for Sprint 4.

---

## Tracking mechanism

This document is the durable state of the project across Claude sessions.

**On every meaningful task or decision:**
1. Tick the relevant `[ ]` → `[x]` in the appropriate sprint section
2. Append to the **Decisions log** if a non-obvious choice was made (with date + rationale)
3. Add to **Open questions** if something is now known to be unknown
4. Update `Last updated:` at the top
5. Commit with `docs(plan): <one-line summary>`

**Cross-session memory** (in `~/.claude/projects/-Users-markforster-t3rm1nu55-monitorplus/memory/`) holds *preferences and patterns*. This file holds *project-specific state*. Don't duplicate.
