# CLAUDE.md — t3rm1nu55-monitorplus

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **For session resumption:** Read this file first, then `PLAN.md` for current sprint state. Most context you need to be productive lives in those two files.

---

## Project identity

**t3rm1nu55-monitorplus** is a deep, low-level system monitor for Apple Silicon Macs, written in Rust. It is built as a personal tool for the author with three planned public-release variants.

The differentiator vs existing tools (macmon, asitop, btop, the existing macOS MCP servers) is **depth**: IOReport + kperf PMU + SMC sensor reads + per-process power inference + macOS pressure-stall equivalent + an MCP-first design for LLM agents. Existing tools either go wide-and-shallow (sysinfo wrappers) or deep on one dimension (macmon for power); none combine deep Apple Silicon access with an LLM-native query interface and a tier-laddered release model.

- **Started:** 2026-04-06
- **Author:** Mark Forster (@t3rm1nu55)
- **License:** Dual MIT / Apache-2.0 (Rust convention)
- **Sister research repo:** [t3rm1nu55-ane-research](https://github.com/t3rm1nu55/t3rm1nu55-ane-research) — tracks AMX/ANE counter exposure via daily autonomous research agent

---

## Architecture

Cargo workspace with 6 crates:

| Crate | Type | Role | Tier |
|---|---|---|---|
| `t3rm-core` | lib | Collector traits, source impls, channel discovery, version-resilience layer | 1 |
| `t3rm-tsdb` | lib | Embedded TSDB: Gorilla compression on `redb` + Last Value Cache | 1 |
| `t3rm-pmud` | bin | Privileged kperf sidecar — root-only, opt-in, unix socket to core | 2 |
| `t3rm-mcp` | bin | MCP server with Tasks primitive, <500-token responses | 1+ |
| `t3rm-tui` | bin | ratatui dashboard | 1+ |
| `t3rm-exporter` | bin | Prometheus `/metrics` endpoint | 1+ |

### Three release variants (Cargo features on `t3rm-core`)

| Variant | Binary name | Features | Drops vs A | Distribution |
|---|---|---|---|---|
| **A** | `t3rm` | `tier3 + research` | nothing — full capability | personal build, manual install |
| **B** | `t3rm-pro` | `tier2 + research` | Endpoint Security events | signed Homebrew tap (future) |
| **C** | `t3rm-public` | `tier1` | kperf PMU, per-process power inference, macOS-PSI | anonymous Homebrew formula (future) |

CI matrix builds **all three variants on every commit**. Breaking variant C with a tier-2-gated symbol is a CI failure on day one. This locks in the feature flag architecture before drift can happen.

### Privilege tiers (the hidden constraint that shapes everything)

| Tier | Source | Exposes | Cost |
|---|---|---|---|
| 1 | IOReport (private dylib via `dlopen`), SMC HID hub, sysinfo, sysctl, libproc-self | CPU/GPU/ANE power, frequency residency, temps, fans, memory, disk, own-user processes | nothing |
| 2 | kperf/kpc (private framework via `dlopen`) | PMU hardware counters: IPC, cache misses, branch mispredictions, thread stalls. Requires root + `kpc_force_all_ctrs_set(1)` | sudo at runtime |
| 3 | Endpoint Security framework | File/process/network flow events | Apple-approved entitlement `com.apple.developer.endpoint-security.client` (must apply) |

---

## Key design rules (from research synthesis on 2026-04-06)

These are non-negotiable conclusions from the academic/technical grounding pass. See `PLAN.md` "Decisions log" for citations.

1. **Channel names are version-fragile.** Apple silently removed the `bandwidth` powermetrics sampler in macOS 13 Ventura, breaking asitop. Every IOReport channel must be runtime-discovered, not hardcoded. The collector enumerates available channels at startup, logs the diff from the expected set, and degrades gracefully.

2. **MCP tool responses target <500 tokens.** Context Rot research (Chroma 2025) shows 30%+ accuracy degradation when LLM context exceeds ~50K tokens, especially for information in the middle. Tool responses return structured summaries — current value, 1-min average, 24h p95, trend direction, anomaly flag — never raw time series. Use the MCP Tasks primitive (2025-11-25 spec) for async historical range queries.

3. **kperf access is mutually exclusive with Instruments.app.** When Instruments runs, it holds the counter lock. Detect at sidecar startup and surface as a user-visible warning.

4. **Multi-cluster chips (M-Pro/Max/Ultra) need special-cased IOReport enumeration.** macmon and socpowerbud both have known attribution bugs here; the IOReport channel structure assumes one P-cluster + one E-cluster. Detect cluster topology at startup and special-case enumeration for >1 cluster of each type.

5. **No vendored reverse-engineered headers.** IOReport, kperf, and ANE private frameworks are loaded via `dlopen` at runtime. Function signatures are declared as `extern "C"` in our own code. We do NOT copy headers from `IOReport_decompile` or similar projects (potential IP risk).

6. **The legacy Rust `smc` crate may not work on Apple Silicon.** Sensors moved to a HID sensor hub on M-series Macs. Follow the `iSMC` tool's HID enumeration approach, not the legacy IOKit SMC path used on Intel.

7. **Per-process network bandwidth is genuinely broken on modern macOS.** `NetworkStatistics.framework`'s kernel socket disappeared in Big Sur with no replacement. Document as a known v1 gap. Possible future paths: Endpoint Security (entitlement gate), `proc_pidfdinfo` polling (root + O(N×M)), `nettop` wrapping (fragile). Don't ship per-process bandwidth in v1.

8. **Per-process power attribution is an open research problem on Apple Silicon.** No published model maps kperf event counters to watts on M-series chips. The McPAT-Calib methodology applies: collect (PMU vector, IOReport energy delta) pairs as training data, fit a regression, ship as a runtime inference layer. This is a v1 feature, not deferred research.

9. **macOS Pressure Stall (PSI) has no public API.** Build it from kperf thread stall counters + sysctl memory pressure + IOReport thermal throttle signals into a unified pressure metric on a 0–100 scale matching Linux PSI semantics.

---

## Conventions

- **Language:** Rust stable, MSRV pinned in `rust-toolchain.toml`
- **Async runtime:** Tokio (single multi-threaded runtime; backpressure-aware bounded channels)
- **TUI:** `ratatui` + `crossterm`
- **Storage:** `redb` (pure Rust, no system deps) with manual Gorilla encoding (delta-of-delta timestamps + XOR floats)
- **MCP:** Use the official MCP Rust SDK once the `Tasks` primitive (2025-11-25 spec) lands; until then, target the spec directly
- **Errors:** `thiserror` for libraries, `anyhow` for binaries, **never** `unwrap()` outside tests
- **Logging:** `tracing` with structured fields, no `println!` in non-test code
- **Commit style:** Conventional Commits (`feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `test:`, `ci:`)
- **CI:** macos-latest, three-variant matrix, fmt + clippy (deny warnings) + build + test
- **No shipping** code that requires SIP-disabled or forbidden entitlements without an explicit feature flag and a runtime warning

---

## Where state lives

| File | Purpose | Update when |
|---|---|---|
| `CLAUDE.md` (this file) | Stable project context — identity, architecture, key design rules | Architecture or rules change |
| `PLAN.md` | Living sprint state — current sprint, task checkboxes, decisions log | Every meaningful task or decision |
| `README.md` | Public-facing project description | Public messaging changes |
| `CONTRIBUTING.md` | Contribution rules — IP-safe FFI policy, commit style | Contribution policy changes |
| `~/.claude/projects/-Users-markforster-t3rm1nu55-monitorplus/memory/` | Cross-session memory: user prefs, design feedback, project facts | New durable preferences or facts |

When picking up this project in a new Claude session:
1. Read this `CLAUDE.md` (auto-loaded)
2. Read `PLAN.md` for current sprint state and decision log
3. Read `MEMORY.md` index for design preferences
4. Check `git log -20` for recent activity
5. Check open issues on `github.com/t3rm1nu55/t3rm1nu55-monitorplus` for blockers
6. Check `t3rm1nu55-ane-research/journal.md` for any actionable research findings since last session

---

## Out of scope (explicit non-goals for v1)

- **Linux/Windows/x86-Mac support.** Apple Silicon only. The whole point is depth on one platform.
- **Per-process network bandwidth.** Documented gap; revisit when Endpoint Security entitlement is obtained.
- **AMX/ANE throughput counters.** Tracked in the sister research repo; out of main repo v1.
- **Fan control / SMC writes.** Read-only. Writes require `thermalmonitord` bypass and paid Developer ID.
- **Web UI.** Use Grafana via the Prometheus exporter. We don't build a web frontend.
- **Cloud/multi-host aggregation.** Single-host, local-first. Pair with Prometheus + remote_write for distributed.
- **Alerting.** Anomaly *flags* are exposed via MCP, but alert rules + delivery are out of scope. Use Grafana/Alertmanager.
