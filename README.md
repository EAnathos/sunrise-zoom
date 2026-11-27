# Sunrise Zoom

**A smooth, configurable replacement for Sunrise's spyglass zoom.**

Sunrise Zoom changes the view only while the player is using a vanilla spyglass. Use the spyglass as usual (right-click by default); the mod animates into its configured magnification and back out when released. It is a client-only native Rust mod, so neither single-player worlds nor multiplayer servers need a server installation.

> **Status:** the spyglass-specific FOV and animation prototype is in progress. It has not yet been verified in a running game.

## Planned behavior

- Keep the vanilla spyglass and its usual use action. There is no separate zoom key and no free-zoom mode.
- Replace the vanilla spyglass magnification with a user-selected value.
- Animate in while the spyglass is in use and animate out on release, including reversals during a transition.
- Keep the spyglass overlay and arm pose provided by the game.
- Keep all settings local to the client. Server owners do not need to install or configure the mod.

The mod is inspired by the configurable transitions and camera feel of [Zoomify](https://modrinth.com/mod/zoomify); it has its own code and assets. Zoomify itself also has a spyglass option, including a mode that overrides the vanilla spyglass zoom.

## Settings

The settings will be exposed on Sunrise's **Options → Mods → Sunrise Zoom → Config** page.

| Setting | Initial default | Planned range | Description |
| --- | ---: | ---: | --- |
| Spyglass magnification | 4× | 2×–20× | Effective zoom while looking through the spyglass. Vanilla is approximately 10×. |
| Transition duration | 0.18 s | 0–0.6 s | Time to enter or leave zoom. Zero makes it instant. |

Settings and defaults may change after in-game checks. The first prototype uses a smooth ease-in/ease-out curve; choosing separate curves and separate in/out durations can follow once the core behavior is stable.

## Multiplayer and server rules

Sunrise Zoom is `side = "client"`. Sunrise does not send client-only mod IDs to a server, so installing it does not require a matching server mod and does not block joining a server. A server owner cannot use this mod alone to set or enforce a player's magnification: local client settings are not an admin policy and can be changed by the player. Follow the server's rules about zoom and client modifications.

The mod applies only during vanilla spyglass use, but servers may still treat any zoom adjustment as disallowed. This feature is not an anti-cheat or server-enforced restriction.

## Technical design

### Runtime flow

1. Sunrise reports whether the local player is using a spyglass through `ComputeFovModifierEvent.player.using`.
2. The client updates a reversible time-based transition toward the configured spyglass zoom ratio, or back to vanilla when the item is released.
3. Sunrise's `compute_fov` callback receives the world FOV after the built-in spyglass zoom. The mod applies the transition ratio there, on rendered frames, leaving hand/HUD FOV on its existing path.
4. Non-spyglass use leaves FOV unchanged.

The vanilla spyglass narrows the field of view to roughly one tenth. The implementation compensates for that built-in factor to produce the configured *effective* magnification. This assumption and the FOV behavior need an in-game check on the pinned Sunrise build.

### Repository layout

```text
.
├── .github/
│   ├── ISSUE_TEMPLATE/       # Bug and feature forms
│   └── workflows/            # Sunrise release workflow
├── assets/sunrise_zoom/lang/ # Settings labels and tooltips
├── docs/
│   ├── architecture.md       # API findings and implementation decisions
│   └── release-checklist.md  # Packaging and registry checklist
├── src/
│   ├── lib.rs                # Sunrise hooks, settings registration, FOV adapter
│   └── animation.rs          # Reversible frame-time transition and easing
├── CONTRIBUTING.md
├── LICENSE
├── README.md
├── Cargo.toml
└── sunrise-mod.toml
```

Keep Sunrise hooks at the boundary and the animation controller independent from the game API. Avoid server hooks, networking, free-zoom keybinds, or private game internals for this feature.

## Build and development

Install Rust and Sunrise's `cargo sunrise-mod` tool. The project pins its target game build in `sunrise-mod.toml`; the tool fetches the matching SDK into the ignored `.sunrise/` folder.

```sh
cargo sunrise-mod check
cargo sunrise-mod build
cargo test
```

### In-game verification

1. Start a world with Sunrise Zoom enabled and use a spyglass.
2. Confirm zoom begins only while using the spyglass and returns on release.
3. Confirm magnification matches the configured effective multiplier and does not compound with vanilla's zoom.
4. Release or re-use the spyglass during an animation; the transition should reverse without a jump.
5. Set transition duration to zero and confirm the view changes instantly.
6. Confirm holding other items does not change FOV.
7. Check the vanilla spyglass overlay, arm pose, hand/item FOV, and non-spyglass world FOV.
8. Disconnect or change worlds while scoped; normal FOV must be restored.
9. Join a multiplayer server without installing the mod server-side; follow that server's client-mod rules.

## Roadmap

- [x] Confirm Sunrise exposes the local spyglass-use state and post-vanilla FOV hooks.
- [x] Implement configurable spyglass magnification and reversible animated transitions.
- [ ] Verify the vanilla zoom compensation and hand/HUD behavior in game.
- [ ] Add separate in/out durations and easing choices if the initial behavior is stable.
- [ ] Prepare icon, registry metadata, release notes, and a signed Sunrise release.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, branch naming, pull requests, commit messages, and architecture guidance.

## License

Code and documentation: [PolyForm Noncommercial License 1.0.0](LICENSE).
