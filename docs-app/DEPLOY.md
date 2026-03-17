# Deploying to GitHub Pages

## Prerequisites

1. Enable GitHub Pages in your repository settings
2. Set the source to "Deploy from a branch" and select `gh-pages` branch

## Build and Deploy

### Option 1: Manual Deployment

```bash
# Build the release version
cd docs
dx build --platform web --release

# The output is in:
# ../target/dx/dioxus-ui-docs/release/web/public/

# Copy the contents to your gh-pages branch
cp -r ../target/dx/dioxus-ui-docs/release/web/public/* /path/to/gh-pages/
```

### Option 2: GitHub Actions (Recommended)

Create `.github/workflows/docs.yml`:

```yaml
name: Deploy Docs

on:
  push:
    branches: [main]
    paths:
      - 'docs/**'
      - 'dioxus-ui-system/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
      
      - name: Install Dioxus CLI
        run: cargo install dioxus-cli --locked
      
      - name: Build Docs
        working-directory: ./docs
        run: dx build --platform web --release
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./target/dx/dioxus-ui-docs/release/web/public
```

## Configuration

The `base_path` in `Dioxus.toml` is set to `/rust-ds/` for this repository. 
Change it if your repository name is different:

```toml
[web.app]
title = "Dioxus UI Documentation"
base_path = "/your-repo-name/"
```

## Local Testing

```bash
cd docs
dx serve --platform web
```

Then open http://localhost:8080/rust-ds/
