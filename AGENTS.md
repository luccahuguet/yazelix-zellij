# Agent Guidelines

Shared Yazelix workflow and release policy live in `../yazelix/AGENTS.md`.
Read that file before changing this repository.

## Local Scope

- This branch rebuilds the temporary Yazelix Zellij fork from upstream commit
  `5254e4fc1dd784ef872644190dc5e2bcb0981bed` for Nova v1.
- Upstream Zellij owns Kitty Image Protocol parsing, storage, placement,
  rendering, replies, resizing, history, fullscreen, and nested-session
  behavior. Do not add a Yazelix graphics passthrough or replay path.
- Use `yazelix_kgp_preview` only as behavior evidence and a rollback ref. Do
  not merge or mechanically replay it.
- Re-derive each Yazelix behavior from the current product contract and retain
  it only when upstream cannot satisfy that contract.
- Keep each retained behavior in a focused commit and avoid unrelated upstream
  cleanup, formatting, dependency changes, or generated-asset churn.

## Verification

- Prefer focused upstream tests for the touched subsystem before broader Cargo
  checks.
- Prove native Kitty through the packaged Yazi -> Zellij -> Mars path before
  changing Yazelix Main to consume this branch.
- Main Yazelix owns exact pinning, installed-package proof, and fresh-session
  dogfood.
