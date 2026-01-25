# Requirements — Syntax Update (EARS format)

## Introduction
Add explicit DSL support for regex, template, address, and named lists so users can express these generators in canonical DADAGEN syntax.

### Requirement 1: Regex generator support
User Story: As a schema author, I want to declare pattern-based string generators using `regexgen(pattern)` so that generated values match a regexp.

Acceptance Criteria:
1. WHEN the parser reads `regexgen("<pattern>")` THEN it SHALL produce an AST `Generator::Regex(RegexGenerator{pattern})`.
2. THE parser SHALL validate that the pattern is a valid regex (or defer to `RegexGenerator::validate`), returning a parse or validation error for invalid patterns.
3. `create_generator` SHALL produce a `RegexDataGenerator` that can generate strings matching the pattern.

### Requirement 2: Template generator support
User Story: As a schema author, I want to declare `template("...{{field}}...")` so I can compose fields.

Acceptance Criteria:
1. WHEN the parser reads `template("...")` THEN it SHALL produce `Generator::Template(TemplateGenerator{template, variables})`.
2. THE validator SHALL reject empty templates or templates containing empty placeholders `{{}}`.
3. `TemplateDataGenerator::generate` SHALL substitute values from the `Context` with placeholder behaviour: existing → value, missing → `[missing:name]`, error → `[error:name]`.

### Requirement 3: Address sub-generators
User Story: As a schema author, I want `address.city` and `address.postcode` shorthand so I can pick address components easily.

Acceptance Criteria:
1. WHEN parser sees `address.<component>` it SHALL produce `Generator::Address(AddressGenerator{component})`.
2. `create_generator` SHALL map address components to `AddressDataGenerator`.

### Requirement 4: Named lists & listFrom
User Story: As a schema author, I want named lists (external data lists) and to reference them by name, e.g., `listFrom("firstnames")`.

Acceptance Criteria:
1. Parser SHALL accept `listFrom("<name>")` (and optionally `list <name>`) and yield `Generator::List(ListGenerator{name})`.
2. `ListDataGenerator` SHALL load named lists via list manager or fallback to cached sample lists.

### Requirement 5: Maintain shorthand English forms
User Story: As a user, I can still write shorthand `number between A and B` and the macro/parser will canonicalize to `number(A,B)`.

Acceptance Criteria:
1. Parser recognizes the English shorthand and outputs canonical AST.

## Non-Functional Requirements
- Tests: Unit tests for parser, AST mapping, generator factory and end-to-end generation.
- Documentation: Update `docs/DADAGEN_SYNTAX.md` and `config/sample_random_dsl.dadagen` with examples.
- Backwards compatibility: Existing config files should still parse; provide migration notes for ambiguous constructs.
