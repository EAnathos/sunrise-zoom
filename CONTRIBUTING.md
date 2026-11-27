# Contributing to Sunrise Zoom

Thanks for helping build Sunrise Zoom. The project can replace vanilla spyglass magnification or add an optional rebindable zoom key, with configurable client-side transitions.

## Before opening an issue

- Search existing issues and pull requests for the same report or proposal.
- Use the issue forms for bugs and feature requests.
- For feature work beyond a small fix, describe the user problem and proposed behavior before implementation.
- Keep the first release focused on the two documented activation modes, transition animation, settings, and reliable FOV restoration. Discuss additional zoom modes separately.

For bug reports, include Sunrise build, mod version or commit, client/server context, reproduction steps, and relevant logs. Do not attach full worlds or private files.

## Development setup

Install Rust and Sunrise's `cargo sunrise-mod` tool. Builds must use the Sunrise version pinned in `sunrise-mod.toml` and the SDK fetched by the tool into `.sunrise/`.

```sh
cargo sunrise-mod check
cargo sunrise-mod build
cargo test
```

The README describes the player-facing features. See [the development guide](docs/development.md) for the manual in-game verification checklist and [the architecture notes](docs/architecture.md) for implementation decisions. If a change affects camera behavior, list the specific manual checks performed in the pull request.

## Branch workflow

- `main` is the only long-lived branch and contains reviewed changes and releases.
- Create short-lived topic branches from the latest `main`, then open a pull request back to `main`.
- Use descriptive prefixes: `feature/`, `fix/`, `docs/`, `refactor/`, `test/`, or `chore/`.
- Examples: `feature/zoom-animation`, `fix/fov-reset-on-disconnect`, `docs/client-settings`, `chore/release-workflow`.
- Do not commit directly to `main`, force-push shared branches, or delete shared branches.
- Keep pull requests focused. Update the README when player-facing behavior changes and `docs/architecture.md` when technical decisions change.

Protect `main` by requiring a pull request and one approval, dismissing stale reviews, requiring resolved conversations, and disallowing force pushes and deletion. An administrator bypass may be used only for repository maintenance or release tooling that requires a direct release push; routine development still follows pull requests.

## Commit messages

Use a concise Conventional Commit subject:

```text
<type>(optional-scope): <imperative summary>
```

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`, and `ci`.

```text
feat(animation): add reversible zoom transitions
fix(camera): restore fov after disconnect
docs: describe client configuration
```

Write subjects in English, use one logical change per commit, and omit the trailing period. Avoid committing generated build output or `.sunrise/` SDK files.

## Architecture guidelines

- Keep the transition controller independent from Sunrise hooks. It accepts current/target ratios and elapsed time, then returns the ratio to apply.
- Keep Sunrise-specific code at the edges: spyglass/key input, settings registration/reads, and FOV application.
- Store only user preferences in Sunrise's client settings. Animation progress and detected spyglass use are transient runtime state.
- Validate config values at load time and clamp magnification/durations to documented safe ranges.
- Treat spyglass release/reuse, disconnects, world changes, and unload as explicit transition/reset paths.
- Use only supported public Sunrise mod APIs. Record API limitations in `docs/architecture.md` and adjust requirements when necessary.
- Avoid server hooks, networking, and mandatory dependencies unless a concrete feature requires them.

## Pull requests

Explain the user-visible change, implementation approach, and any API assumptions. Include linked issues, build/check results, and in-game reproduction or verification steps where relevant. A reviewer should be able to tell what changed and how to verify it without guessing.

## Release process

Releases are prepared from a clean `main` checkout using the Sunrise release tooling once the project is functional and registry-ready. Verify version, changelog, icon, metadata, target game compatibility, and in-game behavior before making a release. The initial release is `0.1.0`; wait for the outstanding issues and in-game checks to be resolved before declaring `1.0.0`. Do not hand-edit generated release assets or signing data.

## License

By submitting a contribution, you agree that it can be distributed under this repository's [PolyForm Noncommercial License 1.0.0](LICENSE). Only submit code, text, and assets that you have the right to contribute.
