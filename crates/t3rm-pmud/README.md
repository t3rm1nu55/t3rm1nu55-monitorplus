# t3rm-pmud

Privileged kperf sidecar — root-only, opt-in binary that reads PMU hardware counters and serves them to the main `t3rm` daemon over a unix socket.

**Status:** Sprint 0 scaffold. Real implementation lands in Sprint 3.

## Why a separate binary

macOS won't let a regular binary acquire kperf counters without root. The standard pattern (used by `powermetrics`, `pmset`, and Instruments.app internally) is a small setuid sidecar that holds privilege, while the main daemon stays unprivileged. Keeping the privileged code isolated means a security audit only has to review this one tiny binary.

## Known gotchas (Sprint 3)

- Instruments.app holds the kperf counter lock — detect and surface as a user-visible warning
- Early M4 revisions have `kpc_set_config` failures — probe and fall back
