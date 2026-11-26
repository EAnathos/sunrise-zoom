# Architecture notes

This document records implementation decisions and findings from the Sunrise API feasibility work. The current state is intentionally provisional until verified in a running client.

## Current decisions

- Mod side: client only; zoom is a local camera preference and should not require server installation.
- Target behavior: interpolate the effective FOV from its live unzoomed value toward a configurable magnification.
- Core state: a reversible time-based transition, separate from the Sunrise event adapter.
- Settings: client-local config; no server config or synchronization.
- Dependency policy: use the pinned Sunrise mod API only unless an issue justifies another dependency.

## API feasibility checklist

- [ ] Confirm the supported FOV/zoom callback and how to transform its value.
- [ ] Confirm whether the held hand/item uses a separate FOV path.
- [ ] Confirm key mapping registration and press/release state delivery.
- [ ] Confirm safe mouse-wheel handling without consuming GUI or inventory scroll.
- [ ] Confirm client configuration read/write and default migration behavior.
- [ ] Confirm reset hooks for lost focus, pause screens, disconnect, world change, and unload.
- [ ] Build and run a minimal prototype on the pinned Sunrise build.

The SDK checkout available during repository setup contains key-mapping registration support and zoom/FOV hook symbols. The exact public event shapes and lifecycle guarantees still need to be verified when implementing against the new repository's pinned SDK.

## Planned component boundaries

| Component | Responsibility |
| --- | --- |
| `config` | Defaults, validation, persisted client preferences, config migration. |
| `input` | Key mapping state, hold/toggle semantics, optional scroll adjustment. |
| `zoom` | Desired state, current progress, target magnification, reversals. |
| `animation` | Easing functions and duration-based interpolation. |
| `camera` | Sunrise FOV callback adapter and restoration behavior. |
| `lib` | Mod entry point, hook wiring, lifecycle reset dispatch. |

Keep components small and create files when their responsibility has actual code. Avoid speculative abstraction layers and private game internals.
