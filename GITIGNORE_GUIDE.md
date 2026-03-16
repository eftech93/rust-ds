# Gitignore Guide

This project uses a hierarchical `.gitignore` structure:

## Root `.gitignore`

Contains rules for:
- **Rust** - `target/`, `Cargo.lock`
- **IDEs** - VS Code, IntelliJ, Vim, etc.
- **OS files** - `.DS_Store`, etc.
- **Dioxus CLI** - `.dx/`, build cache
- **Environment** - `.env`, secrets

## Example-Specific `.gitignore` Files

### `examples/web-csr/.gitignore`
- `node_modules/` - If using npm packages
- `dist/` - Web build output
- `package-lock.json`, `yarn.lock`

### `examples/mobile/.gitignore`
- iOS: `*.ipa`, `*.dSYM`, `ios/build/`
- Android: `*.apk`, `*.aab`, `android/build/`
- Generated Xcode projects

### `examples/desktop/.gitignore`
- macOS: `*.app`, `*.dmg`
- Windows: `*.exe`, `*.msi`
- Linux: `*.deb`, `*.rpm`, `*.AppImage`
- Bundle outputs

## Testing Your Gitignore

Check what's being ignored:

```bash
# See all ignored files
git check-ignore -v **/*

# Check if a specific file is ignored
git check-ignore -v examples/mobile/target

# See untracked files (should show what we want to keep)
git status
```

## Adding New Examples

When creating a new example, consider adding a `.gitignore` if it has:
- Platform-specific build outputs (iOS, Android, etc.)
- Package manager files (npm, yarn, etc.)
- IDE-specific files
- Generated code

Template for new example:

```gitignore
# Example: my-example/.gitignore

# Build outputs
/build/
/dist/

# Generated files
/generated/

# Platform specific
*.platform
```
