# Sunrise Zoom

**Replace spyglass zoom or use an optional animated zoom key.**

Sunrise Zoom offers two client-side modes: replace the vanilla spyglass zoom, or add a separate hold-to-zoom key. Choose the mode in the mod's settings. The default mode uses the spyglass as usual (right-click by default); the optional key mode defaults to `C` and can be rebound in Controls. Both modes animate into the configured magnification and back out when released. Neither single-player worlds nor multiplayer servers need a server installation.

> **Status:** the spyglass-specific FOV and animation prototype is in progress. It has not yet been verified in a running game.

## Planned behavior

- **Replace spyglass zoom (default).** Keep the vanilla spyglass action and replace its magnification with a user-selected value.
- **Separate key mode (optional).** Hold a rebindable key to zoom without a spyglass. While a spyglass is in use, its vanilla zoom remains in effect and the extra key zoom is suspended to avoid stacking.
- Animate in while the selected activation is held and animate out on release, including reversals during a transition.
- Keep the spyglass overlay and arm pose provided by the game.
- Keep all settings local to the client. Server owners do not need to install or configure the mod.

The mod is inspired by the configurable transitions and camera feel of [Zoomify](https://modrinth.com/mod/zoomify); it has its own code and assets. Zoomify itself also has a spyglass option, including a mode that overrides the vanilla spyglass zoom.

## Settings

The settings will be exposed on Sunrise's **Options → Mods → Sunrise Zoom → Config** page.

| Setting | Initial default | Planned range | Description |
| --- | ---: | ---: | --- |
| Zoom mode | Replace spyglass zoom | Two modes | Choose between replacing spyglass zoom and using a separate key. |
| Magnification | 4× | 2×–20× | Effective zoom for the selected mode. Vanilla spyglass is approximately 10×. |
| Transition duration | 0.18 s | 0–0.6 s | Time to enter or leave zoom. Zero makes it instant. |

The separate key defaults to `C` and is rebindable under **Options → Controls → Sunrise Zoom**. It is registered even when spyglass mode is selected, but has no effect until separate key mode is selected.

Settings and defaults may change after in-game checks. The first prototype uses a smooth ease-in/ease-out curve; choosing separate curves and separate in/out durations can follow once the core behavior is stable.

## Multiplayer and server rules

Sunrise Zoom is `side = "client"`. Sunrise does not send client-only mod IDs to a server, so installing it does not require a matching server mod and does not block joining a server. A server owner cannot use this mod alone to set or enforce a player's magnification: local client settings are not an admin policy and can be changed by the player. Follow the server's rules about zoom and client modifications.

The default mode changes FOV during vanilla spyglass use; the optional key mode changes FOV while its key is held. Servers may still treat either behavior as disallowed. These modes are not an anti-cheat or server-enforced restriction.

## Technical design

### Runtime flow

1. Sunrise reports whether the local player is using a spyglass through `ComputeFovModifierEvent.player.using`; its registered key mapping supplies the optional hold input.
2. The client selects the activation from the chosen mode and updates a reversible time-based transition toward the configured zoom ratio.
3. Sunrise's `compute_fov` callback applies the current transition ratio on rendered frames. In spyglass mode it compensates for the built-in spyglass zoom; in key mode it applies a normal-view ratio. Hand/HUD FOV remains on its existing path.
4. Outside the selected activation, FOV is unchanged.

The vanilla spyglass narrows the field of view to roughly one tenth. The implementation compensates for that built-in factor to produce the configured *effective* magnification. This assumption and the FOV behavior need an in-game check on the pinned Sunrise build.

### Repository layout

```text
.
├── .github/
│   ├── ISSUE_TEMPLATE/       # Bug and feature forms
│   └── workflows/            # Sunrise release workflow
├── assets/sunrise_zoom/lang/ # Settings labels, tooltips, and keybind name
├── docs/
│   ├── architecture.md       # API findings and implementation decisions
│   └── release-checklist.md  # Packaging and registry checklist
├── src/
│   ├── lib.rs                # Sunrise hooks, settings/key registration, FOV adapter
│   └── animation.rs          # Reversible frame-time transition and easing
├── CONTRIBUTING.md
├── LICENSE
├── README.md
├── Cargo.toml
└── sunrise-mod.toml
```

Keep Sunrise hooks at the boundary and the animation controller independent from the game API. Avoid server hooks, networking, toggle mode, or private game internals for this feature.

## Build and development

Install Rust and Sunrise's `cargo sunrise-mod` tool. The project pins its target game build in `sunrise-mod.toml`; the tool fetches the matching SDK into the ignored `.sunrise/` folder.

```sh
cargo sunrise-mod check
cargo sunrise-mod build
cargo test
```

### In-game verification

1. In the default mode, use a spyglass and confirm zoom changes only while it is in use.
2. Confirm configured magnification replaces, rather than stacks with, vanilla spyglass zoom.
3. Select separate key mode, rebind the key, and confirm it zooms while held without a spyglass.
4. While in separate key mode, use the spyglass and confirm vanilla spyglass zoom remains unchanged and does not stack with the key zoom.
5. Release the active control during a transition; the view should reverse without a jump.
6. Set transition duration to zero and confirm the view changes instantly.
7. Check the vanilla spyglass overlay, arm pose, hand/item FOV, and non-zoomed world FOV.
8. Disconnect or change worlds while zoomed; normal FOV must be restored.
9. Join a multiplayer server without installing the mod server-side; follow that server's client-mod rules.

## Roadmap

- [x] Confirm Sunrise exposes the local spyglass-use state and post-vanilla FOV hooks.
- [x] Implement spyglass replacement and optional separate-key magnification with reversible transitions.
- [ ] Verify the vanilla zoom compensation and hand/HUD behavior in game.
- [ ] Add separate in/out durations and easing choices if the initial behavior is stable.
- [ ] Prepare icon, registry metadata, release notes, and a signed Sunrise release.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, branch naming, pull requests, commit messages, and architecture guidance.

## License

Code and documentation: [PolyForm Noncommercial License 1.0.0](LICENSE).
