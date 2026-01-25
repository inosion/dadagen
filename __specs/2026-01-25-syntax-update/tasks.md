# Tasks — Syntax Update (implementation plan)

1. Parser & Grammar (Owner: Parser) — Estimated: M
   - Update `dsl.pest` to parse `regexgen("<pattern>")`, implicit template strings (quoted strings with `{{...}}`), `list("<name>")` and keep dot-family notation.
   - Add rules for quoted strings, triple-quoted strings, and escaped sequences for regex patterns.
   - Add unit tests for parser rules.

2. AST & Types (Owner: Core) — Estimated: S
   - Verify `ast.rs` `Generator` enum has variants: `Regex(RegexGenerator)`, `Template(TemplateGenerator)`, `List(ListGenerator)`, `Address(AddressGenerator)`.
   - Add/adjust struct fields where necessary (e.g., TemplateGenerator.variables may include optional generator refs or names).

3. Factory Mapping (Owner: Core) — Estimated: S
   - Ensure parser output maps to AST variants.
   - Verify `create_generator` handles these variants and returns the right `DataGenerator`.

4. Validation and Generators (Owner: Core) — Estimated: S
   - Harden `TemplateDataGenerator::validate` to detect empty placeholders (done).
   - Ensure `RegexDataGenerator::validate` returns meaningful messages for invalid patterns.
   - Add tests for validation errors.

5. Named Lists Integration (Owner: List Manager) — Estimated: M
   - Define `list("name")` semantics and lookup (config path or built-in lists directory).
   - Wire `ListDataGenerator::load_list_data` to the list manager.
   - Add tests for missing list name and successful load.

6. GUI & Emitters (Owner: GUI/JMeter) — Estimated: S
   - Update `dadagen-gui` and `dadagen-jmeter` code that emits DSL to produce canonical forms.
   - Add examples in GUI templates.

7. Examples & Docs (Owner: Docs) — Estimated: S
   - Update `docs/DADAGEN_SYNTAX.md` with canonical examples and migration notes.
   - Update `config/sample_random_dsl.dadagen` to canonical syntax and add a migration snippet showing Scala→DSL.

8. Tests & CI (Owner: QA) — Estimated: M
   - Add parser unit tests, AST roundtrip tests, generator factory tests and end-to-end generation tests using sample DSL files.
   - Run full test suite and fix regressions.

9. Release Notes / Changelog (Owner: Release) — Estimated: XS
   - Document syntax changes and migration notes; include examples.

10. Pull Request & Review
   - Open PR with small logical commits: parser, ast, factory, tests, docs.
   - Ensure CI runs and all tests pass before merging.

## Milestones
- M1 (Parser + AST mapping) — parser rules + tests, AST updates (Tasks 1–2)
- M2 (Factory + Generators) — create_generator mapping + generator tests (Tasks 3–4)
- M3 (Lists + Integration) — named-list integration + GUI changes (Tasks 5–6)
- M4 (Docs + Release) — docs, examples, PR (Tasks 7–10)

## Notes
- Prefer canonical AST outputs; keep parsing shorthands small and unambiguous.
- When in doubt, surface parse-time errors rather than silently canonicalising ambiguous constructs.
