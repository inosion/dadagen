### DadaGen UI (Rust)

The DadaGen UI is a desktop app for Windows, Mac and Linux. It provides a lightweight editor for generating data with file export in common formats.

## Download 

Head over to releases 

- https://github.com/inosion/dadagen/releases


## Usage

- Open or paste a DADAGEN schema in the editor. The editor supports the canonical DSL (implicit templates with `{{...}}`, `regexgen(...)`, `list("name")`, `enum(...)`, `address.city`, etc.).
- Set number of rows to generate and an output filename.
- Choose an output format: CSV, JSON or XML.
- Click Generate — the UI will call the Rust engine and write the output file.

Screenshots

See `assets/` for GUI screenshots used in releases.

## Key features

- Uses the native Rust UI (GTK/Tauri/egui depending on build) for a snappy desktop experience.
- Editor with schema syntax examples and quick validation.
- Supports implicit templates (quoted strings with `{{var}}`), embedded generators, named lists (`list("name")`) and inline enums (`enum("a","b")`).
- Output formats: CSV, JSON, XML.
- Configurable list resource resolution for `list(...)` entries; fallbacks use bundled sample lists.

## Developer notes

- The GUI project is at `dadagen-gui/` — see `dadagen-gui/src/main.rs` for the entry point.
- To iterate quickly during development use `cargo run -p dadagen-gui`.
- The UI emits canonical DSL strings; if integrating other emitters (JMeter, CLI), ensure they use `list("...")` and implicit templates.

### Build and run

- Build (requires Rust toolchain):

```bash
cargo build -p dadagen-gui --release
```

- Run (from workspace root):

```bash
cargo run -p dadagen-gui --release -- <optional-schema-file>
```

## Contribution

- Open issues and PRs in the repository. Add tests for parser or generator changes under `dadagen-core/tests` and UI tests under `dadagen-gui` when applicable.

## Troubleshooting

- If generation fails, enable logs and inspect the schema validation errors reported by the UI. Common issues: empty template placeholders `{{}}`, invalid regex patterns, or missing named lists.

For more examples see `examples`, `config/sample_random_dsl.dadagen` and `docs/DADAGEN_SYNTAX.md`.


