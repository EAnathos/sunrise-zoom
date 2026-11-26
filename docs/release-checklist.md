# Release checklist

- [ ] Changes are reviewed and merged to `main` from `dev`.
- [ ] Sunrise build and supported game version are correct in `sunrise-mod.toml`.
- [ ] `cargo sunrise-mod check` and `cargo sunrise-mod build` succeed.
- [ ] Run the in-game checks listed in the README, including all reset paths.
- [ ] Confirm client-only installation and multiplayer behavior.
- [ ] Update version/release notes through Sunrise's release tooling.
- [ ] Verify icon, repository description, registry metadata, and README links.
- [ ] Confirm the published package contains no local SDK or build artifacts.
