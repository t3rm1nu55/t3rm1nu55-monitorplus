# t3rm-tsdb

Embedded time-series store for t3rm1nu55-monitorplus.

Planned: Gorilla compression (delta-of-delta timestamps + XOR float values) on top of `redb`, with an in-memory Last Value Cache for snapshot queries.

**Status:** Sprint 0 scaffold. Real implementation lands in Sprint 2.
