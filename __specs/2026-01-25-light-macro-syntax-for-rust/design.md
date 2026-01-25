# Light Macro Syntax for Rust

Date: 2026-01-25

TL;DR
-----
Provide a lightweight, no-proc-macro ergonomic surface to generate typed data from schemas. Developers write normal `struct`s (derive `Deserialize`) and call a small macro wrapper `dadagen!(Type, { ... })` plus an inline `schema!{ ... }` DSL to override field generators. Default schemas are provided, and automatically used via a compact `impl_default_schema!` helper; no derive macro required.

Motivation
----------
- Keep ergonomics exactly as:
  - `let people: Vec<People> = dadagen!(People, { count = 100 });`
  - `let people: Vec<People> = dadagen!(People, { 100, schema = schema!{ "age" number between 1 and 100 } });`
- Avoid proc-macros to reduce compile-time complexity and distribution friction.
- Reuse existing generator core: parse/represent schemas, run generators, deserialize into typed structs via Serde.

Goals
-----
- Simple macro-call UX (two small macros only).
- Use normal Rust structs (require `Deserialize` only).
- Allow partial schema overrides: unspecified fields use a default schema for the type.
- Keep runtime merging logic explicit and testable.
- Minimal surface area to teach and maintain.

Non-goals
---------
- Replace a full derive-based codegen that inspects struct fields at compile time.
- Support advanced type features (generic field mapping, custom field transforms) in the MVP.

Public API (examples)
---------------------
- Default schema provider (manual):

```
impl_default_schema!(People => {
    ("name","name"),
    ("age","number(18,80)"),
    ("tax_no","number(1000,9999)")
});
```

- Macro call forms:
  - Use default schema:

```
let people: Vec<People> = dadagen!(People, { count = 100 });
```

  - Override some fields with inline schema:

```
let people: Vec<People> = dadagen!(People, { 100, schema = schema!{ "age" number between 1 and 100 } });
```

Semantics
---------
- `impl_default_schema!` creates a `HasDefaultSchema` impl returning a `Schema` (vector of `FieldSpec`).
- `schema!{ ... }` is a compile-time helper macro creating a `Schema` (limited, predictable syntax).
- `dadagen!` expands to a call that:
  1. Retrieves `T::default_schema()` via `HasDefaultSchema`.
  2. Merges any inline `schema!` override: per-field replacement when names match; new fields appended.
  3. Calls generator core to produce `Vec<serde_json::Value>` records and deserializes each into `T` using `serde_json::from_value`.
- Overrides only affect generator spec for the field; field types remain validated by deserialization.

Merging rules
-------------
- For each default field:
  - If override schema has same name → replace generator spec.
  - Else keep default field.
- Fields present only in override → appended to schema (useful for adding optional fields).
- Conflicting types are detected at deserialization time and surface errors.

Implementation outline
----------------------
- Add small support APIs in `dadagen-core`:
  - `FieldSpec`, `Schema`, `HasDefaultSchema` trait, `impl_default_schema!` macro.
  - `merge_schema(default: &mut Schema, override: &Schema)`.
  - `generate_records(schema: &Schema, count: usize) -> Vec<Value>`.
  - `dadagen_to_struct<T: DeserializeOwned + HasDefaultSchema>(...)` used by macros.
- Add `schema!` and `dadagen!` thin macros in a small, documented module (no proc-macro required).
- Reuse existing generator engine for parsing/generator expressions; accept simple forms in `schema!` (e.g., `name`, `number(min,max)`, or keyword `number between a and b` with the macro canonicalizing to `number(a,b)`).

Testing
-------
- Unit tests:
  - Merge behavior: defaults replaced/retained/appended correctly.
  - Roundtrip: generated JSON deserializes to `T` for common types.
  - Inline DSL parsing for `schema!` forms.
- Integration tests:
  - Example `People` struct with both `dadagen!(People, { count = N })` and override cases.
  - Negative test: override a field with incompatible generator producing deserialization error.

Risks & trade-offs
------------------
- Manual `impl_default_schema!` requires developers to keep schema in sync with struct fields. Mitigation: provide a tiny `cargo` helper or lint to detect mismatches, and clear docs/recipes to convert DSL files to `impl_default_schema!`.
- `schema!` macro complexity: `macro_rules!` limitations mean complex DSL parsing is limited; prefer canonical generator strings (e.g., `number(1,100)`) or supply simple macro forms (`"age" number between 1 and 100`) that expand to canonical string at compile-time.
- Runtime errors (deserialization mismatches) moved from compile-time to runtime—document clearly.

Acceptance criteria
-------------------
- `dadagen!(Type, { count = N })` compiles and returns `Vec<Type>` using default schema.
- `dadagen!(Type, { N, schema = schema!{ ... } })` compiles and applies partial overrides while other fields use defaults.
- Tests cover merging and example usage; docs updated with usage snippets.

Next steps
----------
1. Draft minimal API prototypes in `dadagen-core` (small module + macros).
2. Add `People` example in `examples/` and unit tests.
3. Update docs and sample DSL to show equivalence between DSL files and `impl_default_schema!` snippets.

Optional: expand this document with concrete function/type signatures and a short migration checklist on request.

Recommended approach: External schema files + runtime registry
---------------------------------------------------------
If you don't want users to add derives, attributes, or change their struct definitions, prefer a runtime registry that loads schema definitions from external files (DSL, JSON, or YAML) and maps them to Rust types by name.

How it works
- Store canonical schema files under a known folder in the project, e.g. `schemas/People.dadagen` or `schemas/People.json`.
- At startup (or lazily on first request), the library loads all schema files and registers them in a global registry keyed by the Rust type name (use `std::any::type_name::<T>()` or a stable short-name mapping).
- The `dadagen!(Type, { ... })` macro (or runtime `dadagen_from_type::<T>()` function) looks up the schema by type name and uses it as the default. Inline `schema!{}` overrides are merged at runtime as previously described.

APIs to add
- `SchemaRegistry::load_from_dir(path: &Path) -> Result<()>` — load and parse schema files into registry.
- `SchemaRegistry::register_for_type<T: 'static>(schema: Schema)` — explicit registration API for advanced use.
- `dadagen_from_type::<T>(count, override_schema)` — merge and generate using registered or passed schema.

Benefits
- No code annotations: users keep plain structs (derive `Deserialize` only).
- Clear separation of concerns: schema lives in data files and is editable without recompiling Rust code.
- Easy to script or auto-generate from external DSL sources (CI step can produce schema files from `.dadagen` files).

Trade-offs
- Users must manage external schema files (one file per type), but this can be automated or generated from existing DSL files.
- Mapping between type name and schema file must be stable; consider a small manifest file `schemas/manifest.toml` if you need custom mapping.
- Runtime lookup introduces potential runtime errors when a schema is missing — surface descriptive errors and provide a dev-mode helper that lists missing schemas.

Recommendation
- Implement this runtime registry as the primary integration for the light-macro UX. It's robust, avoids codegen and proc-macros, and fits your requirement that users should not annotate their structs.

