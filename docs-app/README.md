# dioxus-ui-docs

The documentation site for `dioxus-ui-system`, published at
https://eftech93.github.io/rust-ds/. It has a page per component (atoms, molecules,
organisms, charts) with live examples and props tables, plus theme and guide pages.

```bash
cd docs-app
dx serve --platform web     # http://localhost:8080
```

Requires the Dioxus CLI 0.7 and the `wasm32-unknown-unknown` target. Production builds,
the GitHub Pages base path and deployment are covered in the
[Docs site section of CONTRIBUTING.md](../CONTRIBUTING.md#docs-site).
