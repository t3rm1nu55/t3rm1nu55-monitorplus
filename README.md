# t3rm1nu55-monitorplus

Deep, low-level system monitor for Apple Silicon Macs. Written in Rust.

> **Status:** Pre-alpha. Sprint 0 (scaffold) in progress. Not yet usable.
> See [`PLAN.md`](./PLAN.md) for the sprint plan and current state.

---

## What this is

A deep monitoring tool for Apple Silicon Macs that goes further than existing tools by combining:

- **IOReport** for CPU/GPU/ANE power, frequency residency, and temperatures (no sudo required)
- **kperf PMU counters** via a privileged sidecar — IPC, cache misses, branch mispredictions, thread stalls
- **SMC sensor hub** reads via the Apple Silicon HID path (not the legacy Intel-era IOKit interface)
- **Per-process power inference** via a PMU-counter regression trained on paired (PMU vector, IOReport energy delta) data
- **macOS Pressure Stall equivalent** built from kperf stall counters + sysctl memory pressure + IOReport thermal throttle
- **Embedded 24-hour TSDB** with Gorilla compression on `redb`
- **MCP server** for LLM agents — Tasks-primitive-based async historical queries, <500-token summarized responses
- **Prometheus exporter** for Grafana / VictoriaMetrics / etc.
- **ratatui dashboard** for terminal-native viewing

## Why this exists

Existing tools each cover a slice:
- **macmon** is great for IOReport-based power, but not deep on PMU, no MCP, no embedded history
- **asitop** wraps `powermetrics`, breaks when Apple silently removes channels (it broke on macOS 13)
- **btop / bottom** are gorgeous TUIs but generic — no Apple Silicon depth
- **stats** is a Swift menubar app — not a daemon, no exporter
- **Existing MCP servers for macOS** are uniformly shallow `psutil`/`sysinfo` wrappers

None combine deep Apple Silicon access with an LLM-native query interface. That's the gap.

## Release variants

| Variant | Binary | Capabilities | Distribution |
|---|---|---|---|
| **A** | `t3rm` | Everything: IOReport + kperf + Endpoint Security + per-process power + macOS-PSI | Personal build, manual install |
| **B** | `t3rm-pro` | A minus Endpoint Security events | Signed Homebrew tap (planned) |
| **C** | `t3rm-public` | B minus kperf and per-process power inference | Public Homebrew formula (planned) |

CI builds all three variants on every commit.

## Sister project

[`t3rm1nu55-ane-research`](https://github.com/t3rm1nu55/t3rm1nu55-ane-research) — research journal tracking Apple Neural Engine and AMX matrix coprocessor counter exposure. Updated daily by an autonomous research agent.

## License

Dual-licensed under either of:
- Apache License 2.0 ([LICENSE-APACHE](./LICENSE-APACHE))
- MIT License ([LICENSE-MIT](./LICENSE-MIT))

at your option.

## Author

Mark Forster ([@t3rm1nu55](https://github.com/t3rm1nu55))
