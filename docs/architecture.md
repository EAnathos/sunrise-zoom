# Architecture notes

## Decisions

- Mod side: client only. The server does not install, configure, or enforce the local zoom behavior.
- Activation: selectable between replacing vanilla spyglass zoom (default) and a hold-style, rebindable key.
- Magnification: apply a render-time FOV ratio for the selected input; in key mode, suspend key zoom while the spyglass is in use so the two modes do not stack.
- Settings: Sunrise's owner-bound client settings, shown on the mod's own Options > Mods page.
- Animation: a small reversible transition controller sampled from the render-time FOV hook.
- Dependencies: pinned Sunrise mod API only.

## Verified SDK behavior

For the pinned build in `sunrise-mod.toml`:

- `ComputeFovModifierEvent.player.using` exposes whether the local player is using a spyglass.
- `compute_fov_modifier` runs once per client tick before the zoom stage; use it to retarget the transition without changing vanilla gameplay input.
- `compute_fov` receives world FOV after normal calculation and the zoom stage, before projection/Nausea. It also runs for near-plane overlays. Hand/HUD FOV uses another path.
- The `zoom` hook cannot replace the scope behavior: Sunrise documents that the spyglass's own view wins over a native mod's requested `Zoom`.
- Owner-bound key mappings are registered with `Registrar::register_key_mapping` and read through `ClientHost::is_down`.
- Settings can be registered with `Registrar::register_setting` and read through the client host's typed `setting_f64` method.

The API hook catalog is available through `cargo sunrise-mod hooks <term> --all`.

## FOV replacement calculation

Vanilla applies a spyglass FOV factor of approximately `0.1`, equivalent to about 10× magnification. `compute_fov` therefore receives an already narrowed FOV while scoping. To produce an effective magnification `m`, the spyglass mode scales that value by `10 / m`. Separate key mode applies `1 / m` to the ordinary FOV:

```text
spyglass_mode_fov = vanilla_scoped_fov × (10 / configured_magnification)
key_mode_fov      = normal_fov × (1 / configured_magnification)
```

For example, a 4× setting scales vanilla's 10× view by 2.5, yielding the requested 4× effective view. The exact vanilla factor, unrelated camera effects, view overlays, and upper/lower setting limits must be checked in game on the pinned build. Key mode leaves normal spyglass use at the vanilla magnification and suspends its own key zoom while scoping.

The transition stores the current ratio and target ratio. If the target changes during an animation, it starts from the value currently shown, avoiding a jump. The controller is sampled from `compute_fov` so frame rendering is not limited to the client tick rate.

## Component boundaries

| File | Responsibility |
| --- | --- |
| `src/lib.rs` | Hook and key registration, settings, activation detection, FOV adapter. |
| `src/animation.rs` | Reversible time-based transition and easing. |
| `assets/sunrise_zoom/lang/en_us.json` | Settings labels and tooltips. |

Keep the animation controller independent from Sunrise events. Avoid private game internals, server hooks, networking, and a second zoom input for this feature.

## Known checks before release

- Confirm the pinned build's actual vanilla spyglass FOV factor is 0.1.
- Verify both activation modes, key rebinding, configured magnification, and FOV-altering effects.
- Verify `compute_fov` output remains within Sunrise's accepted finite 1–179 degree range.
- Verify fast transitions, release/re-scope reversals, player/world changes, hand rendering, overlays, and non-spyglass views.
- Verify client-only installation on a multiplayer server and document server rule responsibility.
