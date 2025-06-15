# Release Checklist for scripty

## Pre-release Checklist

- [x] Run all tests: `cargo test`
- [x] Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
- [x] Check formatting: `cargo fmt -- --check`
- [x] Update version in `Cargo.toml`
- [x] Update version in documentation (lib.rs)
- [x] Update CHANGELOG.md
- [x] Run examples to ensure they work
- [x] Review and update README.md
- [x] Add appropriate crates.io categories
- [x] Add badges to README

## Release Process

1. **Commit all changes**
   ```bash
   git add -A
   git commit -m "chore: prepare for v0.3.1 release"
   ```

2. **Create and push tag**
   ```bash
   git tag -a v0.3.1 -m "Release version 0.3.1"
   git push origin main
   git push origin v0.3.1
   ```

3. **Publish to crates.io**
   ```bash
   cargo publish --dry-run  # Test first
   cargo publish
   ```

4. **Create GitHub Release**
   - Go to https://github.com/h-zasu/scripty/releases/new
   - Select the `v0.3.1` tag
   - Title: "v0.3.1 - First Public Release"
   - Copy content from RELEASE_NOTES.md
   - Attach any relevant binaries if applicable
   - Publish release

## Post-release Checklist

- [ ] Verify crate appears on crates.io
- [ ] Check that docs.rs builds successfully
- [ ] Test installation: `cargo install scripty`
- [ ] Update any dependent projects
- [ ] Announce release (if applicable)

## Version Bumping for Next Release

After release, bump version for development:
1. Update version in Cargo.toml to next version (e.g., 0.4.0-dev)
2. Add new "Unreleased" section to CHANGELOG.md
3. Commit: `git commit -m "chore: bump version for development"`