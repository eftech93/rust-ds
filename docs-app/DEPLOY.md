# Deploying to GitHub Pages

## Prerequisites

1. Enable GitHub Pages in your repository settings
2. Set the source to "Deploy from a branch" and select `gh-pages` branch

## Build and Deploy

### Option 1: Manual Deployment

```bash
# Build the release version
cd docs-app
dx build --platform web --release

# The output is in:
# ../target/dx/dioxus-ui-docs/release/web/public/

# Copy the contents to your gh-pages branch
cp -r ../target/dx/dioxus-ui-docs/release/web/public/* /path/to/gh-pages/

# IMPORTANT: Create 404.html for SPA routing
cp ../target/dx/dioxus-ui-docs/release/web/public/index.html /path/to/gh-pages/404.html
```

### Option 2: GitHub Actions (Recommended)

Create `.github/workflows/docs.yml`:

```yaml
name: Deploy Docs

on:
  push:
    branches: [main]
    paths:
      - 'docs-app/**'
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
        working-directory: ./docs-app
        run: dx build --platform web --release
      
      - name: Create 404.html for SPA routing
        run: |
          cp ./target/dx/dioxus-ui-docs/release/web/public/index.html ./target/dx/dioxus-ui-docs/release/web/public/404.html
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./target/dx/dioxus-ui-docs/release/web/public
```

## Configuration

The `base_path` in `Dioxus.toml` should be set for GitHub Pages:

```toml
[web.app]
title = "Dioxus UI Documentation"
# For GitHub Pages, set to your repo name with trailing slash:
base_path = "/rust-ds/"
# For local dev, comment out base_path
```

## SPA Routing (Important!)

GitHub Pages doesn't natively support single-page application (SPA) routing. To enable direct URL access to routes like `/atoms/heading`, we use a **404.html fallback**:

1. Copy `index.html` to `404.html` after building
2. GitHub Pages serves `404.html` for unknown routes
3. The Dioxus Router reads the URL and renders the correct page

This is handled automatically in the GitHub Actions workflow above.

## Local Testing

```bash
cd docs-app
dx serve --platform web
```

Then open http://localhost:8080/

## Testing Direct URL Access

After deployment, test direct URL access:
- `https://yourname.github.io/rust-ds/` (home)
- `https://yourname.github.io/rust-ds/atoms/heading` (should work with 404.html fallback)

If direct URLs don't work, ensure the 404.html is properly deployed.
