# Yazelix Zellij Fork

This branch is the clean Nova v1 rebuild of the temporary Yazelix Zellij fork.

| Field | Value |
| --- | --- |
| Upstream base | `zellij-org/zellij` at `5254e4fc1dd784ef872644190dc5e2bcb0981bed` |
| Native Kitty implementation | `0e6e4404027a187f1399a43bf91bdbd13d9636e1` |
| Previous fork | `yazelix_kgp_preview` at `c9e4c246ea47ad06e79f87aca7073fafe89ba8f1` |
| Previous fork role | Behavior evidence and rollback only |
| Current Yazelix runtime delta | None |

Upstream owns the complete Kitty graphics mechanism. Yazelix will add back only
current Nova v1 behavior that cannot be expressed through upstream Zellij.
Appearance startup, terminal titles, and plugin permission behavior must each
be tested against this base before any local implementation is accepted.

The rebuild is accepted for Main only after the exact child revision is
published, the retained diff is reviewed, package and child-plugin checks pass,
and fresh installed Nova sessions prove tiled and popup Yazi previews through
Mars. Until then, Main continues to consume the previous fork revision.
