# Development guide

This guide covers the local development workflow for Sunrise Zoom. Player-facing behavior and settings are described in the [README](../README.md).

## Requirements

- Rust and Cargo
- Sunrise's `cargo sunrise-mod` tool

The target Sunrise build is pinned in `sunrise-mod.toml`. The first check fetches the matching SDK into the ignored `.sunrise/` directory.

## Build and check

From the repository root:

```sh
cargo sunrise-mod check
cargo sunrise-mod build
```

The build writes the mod package to `target/sunrise-mod/sunrise_zoom.smod`.

## In-game verification

Use a separate Sunrise launcher instance with the mod enabled. Check that:

1. Spyglass mode replaces the vanilla magnification and keeps its overlay and arm pose.
2. Key mode zooms without a spyglass; the key can be rebound in Controls.
3. The key does not stack with vanilla spyglass zoom.
4. Spyglass and key modes keep separate magnification settings.
5. In key mode, horizontal and vertical looking both slow in step with the zoom and return smoothly when the key is released.
6. Entering and leaving zoom is smooth at short, long, and zero transition durations.
7. Releasing or pressing the activation during an animation reverses smoothly.
8. Normal view, hand/item FOV, and HUD FOV remain unaffected outside the intended zoom.
9. Switching worlds or disconnecting while zoomed restores the normal view.
10. Client-only installation works in a multiplayer world, subject to that server's rules.

Record the Sunrise build and the selected settings when reporting a camera or input issue.

## Project map

```text
assets/sunrise_zoom/lang/  Settings labels, tooltips, and keybind name
docs/architecture.md       Sunrise API findings and implementation decisions
src/lib.rs                 Sunrise hooks, settings, activation, and FOV integration
src/animation.rs           Reversible time-based zoom transition
```

See [CONTRIBUTING.md](../CONTRIBUTING.md) for branch, commit, and pull request conventions.
