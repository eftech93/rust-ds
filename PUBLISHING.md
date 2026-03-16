# Publishing to crates.io

This guide explains how to publish the `dioxus-ui-system` library to crates.io.

## Prerequisites

1. **crates.io account** - Sign up at https://crates.io/
2. **API Token** - Generate one in your account settings
3. **Git repository** - Your code should be in a git repo

## Step 1: Prepare Your Crate

### 1.1 Update Cargo.toml Metadata

Edit `dioxus-ui-system/Cargo.toml`:

```toml
[package]
name = "dioxus-ui-system"
version = "0.1.0"  # Update version for new releases
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A pure Rust design system for Dioxus with Atomic Design principles"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/dioxus-ui-system"
homepage = "https://github.com/yourusername/dioxus-ui-system"
documentation = "https://docs.rs/dioxus-ui-system"
readme = "README.md"
keywords = ["dioxus", "ui", "design-system", "components", "rust"]
categories = ["gui", "web-programming", "wasm"]
rust-version = "1.70"

# Replace workspace dependencies with explicit versions for publishing
[dependencies]
dioxus = { version = "0.6", features = ["router"] }
serde = { version = "1.0", features = ["derive"] }

[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
```

### 1.2 Create a README.md in the crate directory

```bash
cp README.md dioxus-ui-system/README.md
```

Or create a minimal one:

```markdown
# Dioxus UI System

A pure Rust design system for Dioxus with Atomic Design principles.

## Features

- 🎨 **Atomic Design Architecture** - Components organized as Atoms, Molecules, and Organisms
- 🌗 **Type-Safe Theming** - Comprehensive theme system with light/dark/brand modes
- 🦀 **Pure Rust Styling** - No CSS files - all styles generated in Rust
- 📱 **Multi-Platform** - Works on Web (WASM), Desktop, and Mobile

## Quick Start

```rust
use dioxus_ui_system::prelude::*;

fn App() -> Element {
    rsx! {
        ThemeProvider {
            Button {
                variant: ButtonVariant::Primary,
                "Click me"
            }
        }
    }
}
```

## License

MIT OR Apache-2.0
```

### 1.3 Ensure LICENSE files exist

The LICENSE-MIT and LICENSE-APACHE files should be at the crate root:

```bash
# From project root
cp LICENSE-MIT dioxus-ui-system/
cp LICENSE-APACHE dioxus-ui-system/
```

Or reference the workspace root:

```toml
# In Cargo.toml
license-file = "../LICENSE-MIT"  # Not recommended, use license = "MIT OR Apache-2.0"
```

### 1.4 Add CHANGELOG.md (optional but recommended)

Create `dioxus-ui-system/CHANGELOG.md`:

```markdown
# Changelog

## [0.1.0] - 2024-XX-XX

### Added
- Initial release
- Button, Input, Label, Icon components
- Card, Badge, InputGroup molecules
- Header, DataTable organisms
- Theme system with light/dark modes
- Style builder API
```

## Step 2: Test Before Publishing

### 2.1 Verify the crate builds

```bash
cd dioxus-ui-system

cargo build --release

cargo test

cargo doc --no-deps
```

### 2.2 Dry run publish (test without uploading)

```bash
cargo publish --dry-run
```

This will catch issues like:
- Missing files
- Invalid metadata
- Dependency problems

### 2.3 Check package contents

```bash
cargo package --list
```

This shows what files will be included in the crate.

## Step 3: Publish

### 3.1 Login to crates.io

```bash
cargo login
```

Enter your API token from https://crates.io/settings/tokens

### 3.2 Publish

```bash
cd dioxus-ui-system
cargo publish
```

If successful, your crate is now live at:
`https://crates.io/crates/dioxus-ui-system`

### 3.3 Verify Publication

Check:
- https://crates.io/crates/dioxus-ui-system
- https://docs.rs/dioxus-ui-system (docs build automatically)

## Step 4: Update the Workspace

After publishing, you can use the published version in your examples:

```toml
# In examples/shared/Cargo.toml
[dependencies]
dioxus-ui-system = "0.1.0"  # Instead of path = "..."
```

For development, keep using the path dependency:

```toml
dioxus-ui-system = { path = "../dioxus-ui-system" }
```

## Publishing Updates

### Version Bump

Update version in `Cargo.toml` following SemVer:

- `0.1.0` → `0.1.1` - Bug fixes (patch)
- `0.1.0` → `0.2.0` - New features (minor)
- `0.1.0` → `1.0.0` - Breaking changes (major)

### Tag Releases

```bash
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

### Publish New Version

```bash
cd dioxus-ui-system
cargo publish
```

## Troubleshooting

### "crate already exists"

The crate name is taken. Choose a different name or contact crates.io if it's name squatting.

### "unauthorized"

Your API token is invalid or expired. Generate a new one:
```bash
cargo login
```

### "failed to verify package"

Fix the reported issues and try again. Common issues:
- Missing README.md
- Invalid version format
- Missing license

### "dependency not found"

All dependencies must be published on crates.io. You cannot depend on:
- Path dependencies (use version instead)
- Git dependencies (for publishing)

## Best Practices

1. **Semantic Versioning** - Follow SemVer strictly
2. **Changelog** - Document all changes
3. **Documentation** - Write good rustdoc comments
4. **Tests** - Ensure tests pass before publishing
5. **Examples** - Provide usage examples
6. **CI/CD** - Automate publishing with GitHub Actions

## GitHub Actions for Automated Publishing

Create `.github/workflows/publish.yml`:

```yaml
name: Publish to crates.io

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - uses: actions-rs/cargo@v1
        with:
          command: publish
          args: --manifest-path dioxus-ui-system/Cargo.toml --token ${{ secrets.CARGO_TOKEN }}
```

Add `CARGO_TOKEN` secret in GitHub repository settings.
