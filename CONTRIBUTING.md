# Contributing to Sunrise Zoom

Thanks for helping build Sunrise Zoom. The project replaces the vanilla spyglass magnification with a configurable, animated client-side view.

## Before opening an issue

- Search existing issues and pull requests for the same report or proposal.
- Use the issue forms for bugs and feature requests.
- For feature work beyond a small fix, describe the user problem and proposed behavior before implementation.
- Keep the first release focused on spyglass magnification, transition animation, settings, and reliable FOV restoration. Do not add free zoom or a separate zoom key without first changing the product scope.

For bug reports, include Sunrise build, mod version or commit, client/server context, reproduction steps, and relevant logs. Do not attach full worlds or private files.

## Development setup

Install Rust and Sunrise's `cargo sunrise-mod` tool. Builds must use the Sunrise version pinned in `sunrise-mod.toml` and the SDK fetched by the tool into `.sunrise/`.

```sh
cargo sunrise-mod check
cargo sunrise-mod build
cargo test
```

The README contains the product requirements and manual in-game verification checklist. If a change affects camera behavior, list the specific manual checks performed in the pull request.

## Branch workflow

- `main` contains reviewed releases.
- `dev` is the shared integration branch.
- Create short-lived topic branches from the latest `dev`, then open a pull request back to `dev`.
- Use descriptive prefixes: `feature/`, `fix/`, `docs/`, `refactor/`, `test/`, or `chore/`.
- Examples: `feature/zoom-animation`, `fix/fov-reset-on-disconnect`, `docs/client-settings`, `chore/release-workflow`.
- Promote reviewed changes from `dev` to `main` with a pull request when the release is ready.
- Do not commit directly to `main` or `dev`, force-push shared branches, or delete shared branches.
- Keep pull requests focused. Update README requirements and `docs/architecture.md` when behavior or technical decisions change.

Repository branch protection should require a pull request and one approval for `main` and `dev`, dismiss stale reviews, require resolved conversations, and disallow force pushes and deletion. An administrator bypass may be used only for repository maintenance or release tooling that requires a direct release push; routine development still follows pull requests.

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
- Keep Sunrise-specific code at the edges: spyglass-use detection, settings registration/reads, and FOV application.
- Store only user preferences in Sunrise's client settings. Animation progress and detected spyglass use are transient runtime state.
- Validate config values at load time and clamp magnification/durations to documented safe ranges.
- Treat spyglass release/reuse, disconnects, world changes, and unload as explicit transition/reset paths.
- Use only supported public Sunrise mod APIs. Record API limitations in `docs/architecture.md` and adjust requirements when necessary.
- Avoid server hooks, networking, and mandatory dependencies unless a concrete feature requires them.

## Pull requests

Explain the user-visible change, implementation approach, and any API assumptions. Include linked issues, build/check results, and in-game reproduction or verification steps where relevant. A reviewer should be able to tell what changed and how to verify it without guessing.

## Release process

Releases are prepared from a clean `main` checkout using the Sunrise release tooling once the project is functional and registry-ready. Verify version, changelog, icon, metadata, target game compatibility, and in-game behavior before making a release. Do not hand-edit generated release assets or signing data.

## License

By submitting a contribution, you agree that it can be distributed under this repository's [PolyForm Noncommercial License 1.0.0](LICENSE). Only submit code, text, and assets that you have the right to contribute.
