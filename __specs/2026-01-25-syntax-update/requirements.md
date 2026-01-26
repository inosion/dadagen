# Requirements: DSL Syntax Update (EARS)

This document states testable requirements for the syntax update. Each requirement follows EARS: WHEN [trigger] THEN [system] SHALL [response].

---

### Requirement 1: Field declaration form (colon-only)

User story: As a schema author I want deterministic parsing of fields so that the AST is stable and unambiguous.

WHEN a field is declared in a schema THEN the parser SHALL accept only the colon (`:`) form (e.g. `"name": generator`) and SHALL reject whitespace-only/indentation-only field separators.

Acceptance criteria:
- `"firstname": name.firstname` parses successfully.
- A whitespace-only field line like `"firstname" name.firstname` is rejected by the parser with a clear error.

---

### Requirement 2: Hidden field forms

User story: As a schema author I want to declare seed values for dependency resolution without emitting them in output columns.

WHEN a field is intended as a hidden dependency THEN the syntax SHALL allow the following forms: 

- `hidden : TypeExpr` (global hidden seed)
- `"field_name" hidden: TypeExpr` (hidden field flagged inline)

A field not hidden looks like
- `"field_name": TypeExpr` (normal visible field)

AND the parser SHALL recognise `hidden` semantics and the factory/runtime SHALL treat such fields as non-emitted by default while included in dependency graphs.

Acceptance criteria:
- Parser accepts the three forms and produces an AST node with a `hidden` flag where appropriate.
- `TemplateDataGenerator.dependencies()` includes hidden fields referenced by templates.
- Hidden fields do not appear as emitted output columns unless explicitly requested by downstream emitters.

---

### Requirement 3: Template literals and placeholders

User story: As an author I want to interpolate previously generated fields into strings safely.

WHEN a quoted string contains `{{...}}` THEN the parser SHALL parse it as a `TemplateLiteral` comprised of literal segments and placeholder segments.

Constraints:
- Placeholder content MUST be a field identifier or dotted path matching `[A-Za-z_][A-Za-z0-9_.-]*` (dots are allowed to reference nested fields, e.g. `prop.foo`).
- Embedded generator expressions inside `{{...}}` are NOT permitted and MUST be rejected.
- Empty placeholders `{{}}` MUST be rejected.

A cceptance criteria:
- `"fullname": "{{firstname}} {{surname}}"` produces a `Template` AST with literal and field-ref parts.
- `"bad": "{{}}"` is rejected with an error pointing to the empty placeholder.
-- `"bad2": "{{name.firstname}}"` is accepted and treated as a dotted field reference; the runtime SHALL attempt to resolve `name.firstname` via the dependency/context resolver.

Clarification: dotted fieldnames may be expressed either as a single dotted identifier or via nested objects. Both forms MUST be supported and treated equivalently by the resolver:

- Dotted fieldname form:

```
"field.dotted": generator
```

- Nested object form:

```
"field": { "dotted": generator }
```

The resolver and generator factory SHALL be capable of resolving `field.dotted` whether authored as the dotted identifier or as a nested field in the object form.

---

### Requirement 4: Concatenation

User story: As an author I want to combine string literals and generator tokens.

WHEN an expression uses the `+` operator between `term`s THEN the parser SHALL produce a concatenated AST node representing the ordered sequence of parts.

Acceptance criteria:
- `"display": "User-" + name.firstname + "-" + list("teams")` produces a single concatenation/Template AST node with ordered parts.
- Operator precedence is defined and tested (concatenation binds left-to-right; function calls bind tighter than `+`).

---

### Requirement 5: Lists, enums and generator tokens

User story: As an author I want named lists and enums and a consistent set of generator tokens to use across schemas.

WHEN `list("name")`, `enum(...)` or generator tokens (e.g., `regexgen`, `date.*`, `human.*`) appear THEN the parser SHALL produce canonical AST nodes and the factory SHALL map them to appropriate `DataGenerator` implementations.

Acceptance criteria:
- `list("cities")` maps to a `ListGenerator` AST node; missing lists are reported later by the resolver with a clear message.
- `enum("a","b")` maps to an `EnumGenerator`/choice node.
- The implementation supports the normative token list documented in `design.md`.

---

### Requirement 6: Validation and error reporting

User story: As a developer I want clear validation errors early in the pipeline.

WHEN parsing or validating schemas THEN the system SHALL provide clear, actionable errors including offending token and source location where available.

Acceptance criteria:
- Invalid placeholder characters, empty placeholders, whitespace-field declarations and unresolved lists are reported with clear messages.

---

### Requirement 7: Tests and documentation

User story: As a maintainer I want tests and docs that cover the new syntax.

WHEN the changes are implemented THEN the repo SHALL include parser unit tests (templates, placeholders, concat, lists, enums), AST roundtrip tests, generator unit tests (template substitution and dependencies), and updated `docs/DADAGEN_SYNTAX.md` demonstrating examples.

Acceptance criteria:
- New tests are present and CI passes.
- Documentation examples parse against the new parser.

---

## Non-functional requirements

- Security: templates do not permit arbitrary code evaluation; placeholders are identifiers only.
- Performance: template parsing and generation must be efficient; templates should be represented as pre-parsed parts to avoid repeated scanning.

