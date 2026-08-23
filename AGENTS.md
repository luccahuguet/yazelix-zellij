# Agent Guidelines

Shared Nova workflow and release policy live in the Nova repository's
`AGENTS.md`. Read that file before changing this repository.

## Local Scope

- This branch rebuilds Nova Zellij from exact upstream tag `v0.45.0`, commit
  `13e1c25a2b1ef61d90ecd1765e660c575e90977b`.
- Upstream Zellij owns Kitty graphics. Yazi `v26.8.15` uses Zellij's native
  direct-placement path; do not add a Unicode-placeholder translation layer.
- Retain only the five behaviors named in `YAZELIX.md`: explicit appearance,
  grouped status hints, isolated plugin permissions, stable stack ordering,
  and disconnected-client cleanup.
- Re-derive retained behavior from current upstream source. Do not replay
  unrelated cleanup, formatting, dependency, or generated-asset changes.

## Verification

- Run focused tests for every retained subsystem before broader Cargo checks.
- Rebuild and compare the embedded status-bar WASM after changing its source.
- Main Nova owns exact pinning, installed-package proof, and fresh-session
  dogfood. Do not promote a candidate from this repository alone.
