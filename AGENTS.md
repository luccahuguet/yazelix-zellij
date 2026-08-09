# Agent Guidelines

Shared Yazelix agent workflow and release policy live in the main repo:

- https://github.com/Yazelix/nova/blob/main/AGENTS.md
- In sibling local checkouts, read `../yazelix/AGENTS.md` first

Only Yazelix Zellij temporary-fork guidance belongs here.

## Local Scope

- This repo is a temporary fork of Zellij for the Yazelix Kitty graphics preview path.
- Keep the local delta limited to behavior that cannot be owned cleanly by the
  main Yazelix repo, currently KGP/APC passthrough and the Yazelix terminal
  title shim that needs Zellij's generated session name.
- Treat `upstream` as read-only reference unless the maintainer explicitly asks for upstream work.
- Do not add main Yazelix layout or pane-orchestrator policy here.

## Local Commands

- Prefer focused Cargo checks for touched crates before broad upstream test suites.
- Use upstream Zellij commands when validating upstream-owned behavior.
- Main Yazelix owns packaged-runtime validation of this fork.

## Integration Notes

Main Yazelix uses upstream/nixpkgs Yazi with scoped managed-preview environment for the current KGP path. The removal gate is upstream Zellij support for the required Kitty graphics path, allowing Yazelix to return to upstream Zellij packages.
