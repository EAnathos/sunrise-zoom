# Sunrise Zoom

**A smooth, configurable client-side zoom for Sunrise.**

Sunrise Zoom is a native Rust mod planned around the ease of a hold-to-zoom control and the polished transitions, fine control, and camera options popularized by mods such as [Zoomify](https://modrinth.com/mod/zoomify). It is an independent project: it will use its own implementation, settings, name, and visual identity.

> **Status:** project specification and repository scaffold. The zoom behavior has not been implemented yet.

## Goals

- Add a reliable, rebindable zoom key with hold and toggle modes.
- Animate zoom in and out smoothly, with configurable duration and easing.
- Let players set a default zoom level and optionally adjust it while zoomed.
- Keep settings on the client; servers should not need the mod.
- Restore camera/FOV state cleanly when the key is released, a menu opens, or the player changes worlds.
- Keep the base experience small and approachable, while allowing optional camera and presentation controls.

## Planned features

### First playable release

- **Client-only operation.** No server installation or server configuration required.
- **Rebindable zoom key.** Hold to zoom by default; configurable toggle mode as an alternative. The initial suggested key is `C`, subject to Sunrise's existing bindings and user preference.
- **Animated transitions.** Independent zoom-in and zoom-out durations, with `Instant`, `Linear`, and a useful set of sine, quadratic, and cubic easing curves.
- **Adjustable zoom.** Choose a default magnification and, if supported by Sunrise input hooks, change it with the mouse wheel while zoom is active.
- **Local settings.** Persist key mode, magnification, transition settings, and input preferences in a client config.
- **Safe lifecycle.** Cancel zoom and reset transient state on disconnect, world change, pause/menu transitions, focus loss, and mod disable/unload.

### Follow-up options

These are candidates, not promises for the first release. Each needs a separate issue and compatibility review.

- Separate primary and secondary zoom bindings, with independent settings for creators.
- A cinematic camera mode that smooths camera movement while zoomed.
- Sensitivity scaling based on current zoom amount.
- View-bobbing scaling, if Sunrise exposes a safe client hook.
- Optional spyglass behavior, including whether the item is required and whether its overlay/sounds are shown.
- Hide-HUD-while-zoomed option for the secondary binding.
- Named presets and import/export of settings.
- A small settings screen or client command, if Sunrise provides a stable supported interface.

## Behavior and settings

The target magnification is a multiplier relative to the player's current normal FOV, not a hard-coded replacement FOV. The animation interpolates between the live unzoomed value and that target. Releasing the zoom control reverses from the current animation value, so quick presses do not cause a jump. Changing the configured zoom amount while zoomed should transition to the new target smoothly.

Initial setting proposal:

| Setting | Initial behavior | Notes |
| --- | --- | --- |
| Zoom key | `C` | Rebindable through Sunrise controls if the API supports registration. |
| Activation mode | Hold | Toggle is an accessibility option. |
| Magnification | 4× | Final range and meaning to be verified against Sunrise's FOV API. |
| Zoom-in duration | 0.18 s | Zero means instant. |
| Zoom-out duration | 0.14 s | Separate from zoom-in. |
| Easing | Ease out cubic in; ease in cubic out | Configurable independently. |
| Scroll adjustment | Enabled while zoomed | Must not steal inventory/GUI scrolling. |
| Affect hand/item FOV | Enabled | Option to keep the held item at normal FOV if supported. |

The values above are starting defaults for the design. They may change after the first in-game prototype and accessibility review.

## Technical design

### Runtime flow

1. Read the client's zoom input and current settings.
2. Update a small zoom controller with the desired state and elapsed frame time.
3. Evaluate the selected easing curve to produce a normalized animation value from `0` to `1`.
4. Apply the interpolated zoom through Sunrise's supported client FOV/zoom hook.
5. Restore normal camera behavior whenever zoom is inactive or the client lifecycle changes.

The controller should be independent from Sunrise event wiring. Its inputs and outputs should be plain values so its transition behavior can be reasoned about and later covered by unit tests.

### Repository layout

```text
.
├── .github/
│   ├── ISSUE_TEMPLATE/       # Bug and feature forms
│   └── workflows/            # Release workflow generated/used by Sunrise tooling
├── docs/
│   ├── architecture.md       # Implementation decisions and API findings
│   └── release-checklist.md  # Packaging and registry checklist
├── src/
│   ├── lib.rs                # Sunrise mod entry point and hook registration
│   ├── config.rs             # Client settings, defaults, validation, persistence
│   ├── input.rs              # Key state, activation mode, scroll input
│   ├── zoom.rs               # Zoom state machine and target magnification
│   ├── animation.rs          # Time-based interpolation and easing functions
│   └── camera.rs             # Adapter between the controller and Sunrise camera/FOV hooks
├── CONTRIBUTING.md
├── LICENSE
├── README.md
├── Cargo.toml
└── sunrise-mod.toml
```

Modules can be combined when the implementation is small; this tree describes responsibilities, not a requirement to create empty files. Avoid putting the full controller, settings, and Sunrise callbacks into one file.

### API feasibility gate

Before implementation, confirm against the pinned Sunrise SDK and a small client prototype:

- Which callback changes the effective FOV, and whether its value includes the held hand/item.
- How client key mappings are registered and read (including press/release semantics).
- Whether mouse-wheel input can be intercepted only while zooming and outside screens.
- How client config is declared, loaded, saved, and reloaded.
- Which lifecycle hooks reliably cover menus, focus loss, disconnects, and world changes.
- Whether a config/settings screen is supported without a dependency.

Record findings in [`docs/architecture.md`](docs/architecture.md). If a hook is unavailable, adjust the feature scope before building a workaround that depends on private or unstable internals.

## Build and development

Install Rust and Sunrise's `cargo sunrise-mod` tool. The project will pin its target Sunrise build in `sunrise-mod.toml`; the tool will fetch the matching SDK into the ignored `.sunrise/` folder.

Expected commands once the native scaffold is in place:

```sh
cargo sunrise-mod check
cargo sunrise-mod build
cargo test
```

Manual in-game checks for the first playable build:

1. Confirm the default key zooms in and releasing it zooms out.
2. Rebind the key and check for conflicts with Sunrise controls.
3. Try repeated quick presses and reverse direction mid-transition; the view should remain continuous.
4. Adjust magnification and verify the new target is reached smoothly.
5. Open menus, alt-tab, disconnect, and load another world during zoom; normal FOV must be restored.
6. Check first-person hand/item rendering and third-person camera behavior.
7. Confirm the mod is not required on a multiplayer server.

## Roadmap

- [ ] Verify client key, FOV, config, and lifecycle APIs in the pinned Sunrise SDK.
- [ ] Build a minimal client prototype that can enter and leave zoom.
- [ ] Add the animated zoom state machine and easing options.
- [ ] Add local settings and scroll adjustment.
- [ ] Test lifecycle resets, hand FOV, world changes, and multiplayer client behavior in game.
- [ ] Prepare icon, registry metadata, release notes, and a signed Sunrise release.

## Inspiration and attribution

Zoomify demonstrates how a zoom feature can pair a simple keybind with adjustable magnification, scroll control, transitions, and optional camera behavior. This project uses that feature set as product inspiration only. It does not copy Zoomify's source code, artwork, text, or branding. Zoomify is by its own authors and is available on [Modrinth](https://modrinth.com/mod/zoomify).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, branch naming, pull requests, commit messages, and repository architecture guidance.

## License

Code and documentation: [PolyForm Noncommercial License 1.0.0](LICENSE).
