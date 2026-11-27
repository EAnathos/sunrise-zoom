# Release checklist

- [ ] Changes are reviewed and merged to `main` from a topic branch.
- [ ] Sunrise build and supported game version are correct in `sunrise-mod.toml`.
- [ ] `cargo sunrise-mod check` and `cargo sunrise-mod build` succeed.
- [ ] Run the in-game checks listed in [the development guide](development.md), including all reset paths.
- [ ] Confirm client-only installation and multiplayer behavior.
- [ ] Update version/release notes through Sunrise's release tooling.
- [ ] Verify icon, repository description, registry metadata, and README links.
- [ ] Confirm the published package contains no local SDK or build artifacts.

The first public version is `0.1.0`. Hold `1.0.0` until the tracked issues and required in-game checks are resolved.
