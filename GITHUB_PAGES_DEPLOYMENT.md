# Deploying Dioxus UI to GitHub Pages

This guide explains how to deploy the Dioxus UI design system showcase to GitHub Pages.

## Prerequisites

1. A GitHub repository (this project is assumed to be at `github.com/<username>/rust-ds`)
2. GitHub Pages enabled in your repository settings

## Setup Steps

### 1. Enable GitHub Pages

1. Go to your repository on GitHub
2. Click **Settings** → **Pages** (in the left sidebar)
3. Under **Build and deployment**:
   - Source: Select **GitHub Actions**

### 2. Configure Base Path (Important!)

Edit `examples/web-csr/Dioxus.toml` and update the `base_path`:

```toml
[web.app]
title = "Dioxus UI Design System"
# Update this to match your repository name:
base_path = "/your-repo-name/"
```

For example, if your repo is `github.com/johndoe/my-design-system`:
```toml
base_path = "/my-design-system/"
```

If deploying to a custom domain or username.github.io, set:
```toml
base_path = "/"
```

### 3. Deploy

The deployment happens automatically on every push to `main` or `master` branch.

To deploy manually:
1. Go to **Actions** tab in your GitHub repository
2. Select **Deploy to GitHub Pages** workflow
3. Click **Run workflow**

### 4. Access Your Site

After deployment (usually takes 1-2 minutes), your site will be available at:

```
https://<username>.github.io/rust-ds/
```

Replace `<username>` with your GitHub username and `rust-ds` with your repository name.

## Local Development

To test the web build locally before deploying:

```bash
cd examples/web-csr

# Install Dioxus CLI if not already installed
cargo install dioxus-cli

# Serve locally
dx serve --platform web

# Or build for production
dx build --release

# The built files will be in examples/web-csr/dist/
```

## Troubleshooting

### 404 Errors or Missing Assets

If you see a 404 error or missing styles, check:

1. **Base path**: Ensure `base_path` in `Dioxus.toml` matches your repository name exactly
2. **Case sensitivity**: Repository names are case-sensitive
3. **Trailing slash**: Include trailing slash in base_path (e.g., `/rust-ds/` not `/rust-ds`)

### Build Failures

If the GitHub Actions build fails:

1. Check the Actions logs for specific errors
2. Ensure all dependencies compile: `cargo check --all`
3. Verify `wasm32-unknown-unknown` target is installable

### Changes Not Appearing

1. GitHub Pages has a cache - changes may take a few minutes to appear
2. Try hard refresh: `Ctrl+Shift+R` (Windows/Linux) or `Cmd+Shift+R` (Mac)
3. Check the deployment timestamp in the GitHub Actions tab

## Custom Domain (Optional)

To use a custom domain:

1. Add a `CNAME` file to `examples/web-csr/public/CNAME` with your domain
2. Configure DNS with your provider (A records to GitHub IPs)
3. Update `base_path` in `Dioxus.toml` to `/`
4. Enable custom domain in GitHub Pages settings

## What's Deployed

The deployed site includes:

- **Component Showcase**: All 60+ UI components (buttons, inputs, cards, steppers, etc.)
- **Theme System**: 7 preset themes with live switching
- **Layout System**: 4 layout types (Sidebar, TopNav, Drawer, FullWidth)
- **Card Organisms**: 10+ card variations
- **Stepper Components**: Horizontal, vertical, wizard steppers

All components are interactive and fully functional in the browser!

## Build Configuration

The deployment uses:

- **Rust**: Latest stable toolchain
- **Target**: `wasm32-unknown-unknown`
- **Build command**: `dx build --release`
- **Output directory**: `examples/web-csr/dist`
- **CI/CD**: GitHub Actions (`.github/workflows/deploy.yml`)

## Security Notes

- The site runs entirely client-side (WASM in browser)
- No server-side code is executed
- All assets are static files
- Safe to deploy on public GitHub Pages
