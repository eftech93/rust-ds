# Publishing Checklist

## Before Publishing

- [ ] Update `authors` in `Cargo.toml` with your name and email
- [ ] Update `repository` in `Cargo.toml` with your actual GitHub URL
- [ ] Update `description` if needed
- [ ] Update version in `Cargo.toml` (e.g., 0.1.0)
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Ensure all tests pass: `cargo test`
- [ ] Ensure docs build: `cargo doc`
- [ ] Commit all changes to git

## Publishing Steps

### 1. Login to crates.io

```bash
cargo login
```

Enter your API token from https://crates.io/settings/tokens

### 2. Test the Package

```bash
cd dioxus-ui-system
cargo publish --dry-run
```

### 3. Publish (Manual)

```bash
cd dioxus-ui-system
cargo publish
```

### Or Use the Publish Script

```bash
./scripts/publish.sh
```

### 4. Verify

- [ ] Crate appears at: https://crates.io/crates/dioxus-ui-system
- [ ] Documentation builds at: https://docs.rs/dioxus-ui-system

## After Publishing

- [ ] Tag the release: `git tag -a v0.1.0 -m "Release v0.1.0"`
- [ ] Push tag: `git push origin v0.1.0`
- [ ] Create GitHub release with notes
- [ ] Announce on social media (optional)

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "already exists" | Version already published, bump version |
| "unauthorized" | Run `cargo login` again |
| "dirty" | Commit changes first or use `--allow-dirty` |
| "verify failed" | Fix the reported errors |

## Quick Reference

```bash
# Check what will be published
cargo package --list

# Dry run
cargo publish --dry-run

# Actually publish
cargo publish
```
