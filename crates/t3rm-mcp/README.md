# t3rm-mcp

MCP (Model Context Protocol) server binary. Exposes system metrics to LLM agents via a narrow, well-described tool set.

**Status:** Sprint 0 scaffold. Real implementation lands in Sprint 2.

## Planned tools

| Tool | Purpose | Token budget |
|---|---|---|
| `system_snapshot` | All subsystems at current second | <200 tokens |
| `component_detail(component, window_minutes)` | Deep dive on one subsystem | <500 tokens |
| `process_top(metric, n, window_minutes)` | Top N processes by metric | <500 tokens |
| `anomaly_scan()` | Only series currently outside their 1h baseline | <500 tokens |

All responses return pre-computed summaries (current value, 1-min average, 24h p95, trend direction, anomaly flag) rather than raw time series. Historical range queries use the MCP Tasks primitive (2025-11-25 spec) for async durable requests.

## Why <500 tokens

[Context Rot research (Chroma, 2025)](https://research.trychroma.com/context-rot) shows frontier LLMs lose ~30% accuracy on information in the middle of long contexts. Compact, structured tool responses keep the agent's context window usable for reasoning, not buried under raw metric dumps.
