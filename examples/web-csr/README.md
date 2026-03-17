# Web Client-Side Rendering Example

This example runs the Dioxus UI System in a web browser using WebAssembly.

## Prerequisites

Install the Dioxus CLI:
```bash
cargo install dioxus-cli@0.6.3
```

## Running the Example

### Development Server (with hot reload)

```bash
cd examples/web-csr
dx serve --platform web
```

Then open http://localhost:8080 in your browser.

### With Hot Reload

```bash
dx serve --platform web --hot-reload
```

### Production Build

```bash
# Build for production
dx build --platform web --release

# Output will be in dist/
```

## Why not `cargo run`?

The web-csr example compiles to **WebAssembly (WASM)**, not a native binary. 
`cargo run` would try to run it as a desktop application, which won't work.

The `dx serve` command:
1. Compiles the Rust code to WASM
2. Starts a development server
3. Serves the WASM file to your browser
4. Provides hot reload for development

## Troubleshooting

### "dx: command not found"
Install the Dioxus CLI:
```bash
cargo install dioxus-cli@0.6.3
```

### Port already in use
Change the port:
```bash
dx serve --platform web --port 3000
```

### Build errors
Make sure you have the wasm32 target:
```bash
rustup target add wasm32-unknown-unknown
```

## Deploying to GitHub Pages

This example is configured for automatic deployment to GitHub Pages.

### Setup

1. Go to your repository Settings → Pages
2. Set Source to "GitHub Actions"
3. Update `Dioxus.toml` with your repository name:
   ```toml
   [web.app]
   base_path = "/your-repo-name/"
   ```

### Automatic Deployment

The site deploys automatically on every push to `main`. The workflow is defined in `.github/workflows/deploy.yml`.

### Manual Deployment

To build locally for deployment:

```bash
# Build for production
dx build --release

# The dist/ folder contains the static files ready for deployment
```

See [GITHUB_PAGES_DEPLOYMENT.md](../../GITHUB_PAGES_DEPLOYMENT.md) for detailed instructions.
