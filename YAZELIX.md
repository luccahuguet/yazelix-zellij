# Yazelix Zellij Fork

This branch is the clean Nova v1 rebuild of the temporary Yazelix Zellij fork.

| Field | Value |
| --- | --- |
| Upstream base | `zellij-org/zellij` at `5254e4fc1dd784ef872644190dc5e2bcb0981bed` |
| Native Kitty implementation | `0e6e4404027a187f1399a43bf91bdbd13d9636e1` |
| Previous fork | `yazelix_kgp_preview` at `c9e4c246ea47ad06e79f87aca7073fafe89ba8f1` |
| Previous fork role | Behavior evidence and rollback only |
| Current Yazelix runtime delta | Explicit startup theme mode |

Upstream owns the complete Kitty graphics mechanism. Yazelix will add back only
current Nova v1 behavior that cannot be expressed through upstream Zellij.

`--theme-mode dark|light` selects `theme_dark` or `theme_light` before a new
session's first render. An explicitly themed session ignores ambient terminal
theme reports while retaining manual theme actions and its selected mode across
configuration reloads. Omitting the option keeps upstream behavior. The mode is
passed only through the existing server bootstrap command; the client-server
wire contract is unchanged.

The old terminal-title prefix has no current Nova or Mars consumer and is not
retained. Plugin permissions remain upstream-owned unless focused proof finds a
v1 gap. Late-plugin theme replay and the accepted three-island status bar remain
separate focused slices.

This startup delta is removable when upstream provides equivalent explicit
new-session theme selection, ambient-report authority, manual switching, and
reload behavior.

The rebuild is accepted for Main only after the exact child revision is
published, the retained diff is reviewed, package and child-plugin checks pass,
and fresh installed Nova sessions prove tiled and popup Yazi previews through
Mars. Until then, Main continues to consume the previous fork revision.
