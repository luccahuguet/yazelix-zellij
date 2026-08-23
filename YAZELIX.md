# Yazelix Zellij Fork

This branch is the clean Nova rebuild of the temporary Zellij fork.

| Field | Value |
| --- | --- |
| Upstream base | `zellij-org/zellij` tag `v0.45.0` at `13e1c25a2b1ef61d90ecd1765e660c575e90977b` |
| Previous fork | `yazelix_native_kitty_v1` at `bbccdea6eda81f314151160f9b3f8882a26478ec` |
| Previous fork role | Behavior evidence and rollback only; do not replay mechanically |
| Current Yazelix runtime delta | Explicit startup theme mode, late-plugin theme replay, three-island status hints, isolated permission-cache selection, stable stacked-pane order, and upstream disconnected-client plugin cleanup |

Upstream owns the complete Kitty graphics mechanism. Yazi `v26.8.15` detects
Zellij and uses its native direct-placement path before falling back to Sixel.
The previous Unicode-placeholder translator is intentionally absent.

`--theme-mode dark|light` selects `theme_dark` or `theme_light` before a new
session's first render. An explicitly themed session ignores ambient terminal
theme reports while retaining manual theme actions and its selected mode across
configuration reloads. Omitting the option keeps upstream behavior. The mode is
passed only through the existing server bootstrap command; the client-server
wire contract is unchanged.

When a plugin subscribes to host-theme changes after the session mode is known,
the screen sends the current mode only to that plugin and client. No mode is
sent before the session has one.

In normal mode, the native one-line status bar groups mode actions by their
actual modifiers. Yazelix's bindings therefore render distinct `Ctrl-Alt` and
`Ctrl` islands while the existing upstream renderer retains ownership of the
right-aligned `Alt` actions, compact fallback, and interaction ranges. Missing
actions are omitted instead of disrupting modifier detection.

Zellij embeds the tracked `zellij-utils/assets/plugins/status-bar.wasm` in its
binary. After changing the status-bar source, rebuild that one asset with
`cargo build --release --target wasm32-wasip1 -p status-bar`, copy the result
from `target/wasm32-wasip1/release/status-bar.wasm`, and verify the two files
are byte-identical before packaging.

The old terminal-title prefix has no current Nova or Nova Rio consumer and is not
retained. Fresh-session dogfood proved that background bundled plugins can
remain pending behind inaccessible upstream permission prompts, blocking popup
and pane-orchestrator pipes. When no explicit call-site path is provided, the
fork therefore accepts `ZELLIJ_PLUGIN_PERMISSIONS_CACHE`; Yazelix points it at
an isolated cache containing grants for its exact packaged plugins. Explicit
paths and the default global cache keep upstream precedence and behavior.

The appearance delta is removable when upstream provides equivalent explicit
new-session theme selection, ambient-report authority, manual switching, reload
behavior, and current-state delivery to late plugin subscribers.

The status-bar delta is removable when upstream groups visible mode actions by
their actual modifiers without losing its native secondary actions or compact
behavior.

The permission seam is removable when upstream can make background plugin
requests visible, ordered, granted, and replayed without blocking CLI pipes.

Stack focus and removal preserve each surviving pane's logical identity, then
compact positions after a close. This keeps visible stack order stable when a
pane is opened or closed and when Yazelix reapplies its sidebar swap layout.
The delta is removable when upstream resolves
[#2669](https://github.com/zellij-org/zellij/issues/2669) and
[#4084](https://github.com/zellij-org/zellij/issues/4084) with equivalent
ordering behavior.

The fork carries the two commits from upstream
[#5272](https://github.com/zellij-org/zellij/pull/5272). A replacement client
gets a fresh ID, and after its plugins start, disconnected client-specific
plugin instances are removed so stale orchestrators cannot process later pipes.
This delta is removable when that pull request or equivalent cleanup lands
upstream. Simultaneously connected clients retain upstream broadcast behavior.

The rebuild is accepted only after the exact child revision is published, the
retained diff is reviewed, package and child-plugin checks pass, and fresh
installed Nova sessions prove tiled and popup Yazi previews through Nova Rio.
Until then, Nova channels continue to consume the previous fork revision.
