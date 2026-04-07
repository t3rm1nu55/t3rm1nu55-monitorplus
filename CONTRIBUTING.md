# Contributing to t3rm1nu55-monitorplus

Thanks for your interest. A few ground rules before you open a PR.

## IP-safe FFI policy (non-negotiable)

This project reaches into private macOS APIs (IOReport, kperf, the SMC HID sensor hub). To protect the project from intellectual property risk:

1. **Private framework function signatures are declared as `extern "C"` in our own code.** We do NOT copy header files from projects that publish reverse-engineered Apple headers (e.g., `IOReport_decompile`). If you need a function signature, read it from the public symbol table of the framework and re-declare it manually.
2. **Private frameworks are loaded at runtime via `dlopen`**, not static-linked against Apple's proprietary `.tbd` stubs. This is the pattern used by `macmon`, `asitop`, `socpowerbud`, and Apple's own internal tools.
3. **Do not vendor code from proprietary or incompatibly-licensed sources.** If you vendor an MIT-licensed FFI module, preserve the original copyright header and add an attribution entry to the relevant crate's README.

## Privilege tiers

Every new metric source must declare its tier:

- **Tier 1** — no sudo, no entitlements (IOReport, SMC reads, sysinfo, libproc-self)
- **Tier 2** — root required (kperf PMU counters)
- **Tier 3** — Apple-approved entitlement (Endpoint Security, Network Extension)

Gate tier-2 and tier-3 code behind the corresponding Cargo features (`tier2`, `tier3`). The CI matrix builds all three release variants on every commit, so a tier-gated symbol leaking into a tier-1-only code path is caught immediately.

## Channel discovery & version resilience

Apple silently removed the `bandwidth` powermetrics sampler in macOS 13, breaking asitop. **Every IOReport channel must be runtime-discovered, not hardcoded.** New sources that read a channel must register with the discovery layer and degrade gracefully when the channel is absent.

## Commit style

Conventional Commits: `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `test:`, `ci:`, `perf:`. Keep the subject under 72 characters. Explain the *why* in the body when the change isn't self-evident.

## Testing

- Unit tests in each crate under `#[cfg(test)]`
- Integration tests in `tests/` once there's enough real collector code
- `cargo clippy -- -D warnings` must pass
- No `#[allow(...)]` without a comment explaining why

## Before you open a PR

1. Update `PLAN.md` if the work moves a sprint task forward or introduces a decision
2. Append to the Decisions log if your work makes a non-obvious architectural choice
3. Make sure the CI matrix (variants A/B/C) still passes locally: build each binary with the three feature combinations before pushing
