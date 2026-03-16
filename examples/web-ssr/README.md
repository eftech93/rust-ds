# Web Server-Side Rendering Example

This example demonstrates Dioxus UI System with server-side rendering using Axum.

## How it Works

The HTML is rendered on the server using `dioxus::ssr::render_element()` and sent to the browser. This provides:

- **Faster initial page load** - HTML is ready immediately
- **Better SEO** - Search engines can crawl the content
- **Progressive enhancement** - Works without JavaScript

## Running

```bash
cd examples/web-ssr
cargo run
```

The server will try these ports in order: `3000, 3001, 3002, 3003, 8080`

Check the console output for the actual URL, e.g.:
```
✅ SSR server running on http://localhost:3000
```

Then open the URL in your browser.

## Port Already in Use?

If port 3000 is taken, the server will automatically try the next available port (3001, 3002, etc.).

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Browser   │────▶│ Axum Server │────▶│   Dioxus    │
│             │◀────│   (Rust)    │◀────│   (SSR)     │
└─────────────┘     └─────────────┘     └─────────────┘
```

## Production Deployment

For production, you might want to:

1. Set a fixed port via environment variable
2. Add HTTPS/TLS support
3. Put behind a reverse proxy (nginx, Caddy)

Example with environment variable:
```rust
let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
```
