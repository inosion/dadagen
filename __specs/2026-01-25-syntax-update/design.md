# Syntax Update: Add Regex, Template, Address and Named Lists

## Overview

This spec describes changes to the DADAGEN DSL to add and clarify syntax for:
- `regex` / `regexgen` pattern-based string generators
- implicit template expressions that support `{{...}}` substitution
- `address` sub-generators (for components such as `address.city` and `address.postcode`)
- Named lists and `list("<name>")` expressions (reusable list resources)

Goals:
- Extend the parser to recognise implicit template literals and concatenation (+) so authors can embed generator expressions inside strings and compose generators directly.
- Produce canonical AST nodes for these constructs so existing generator factories (`create_generator`) can be reused.
- Validate template placeholders early and surface clear errors for malformed templates.
- Update documentation and examples to prefer the simpler implicit template syntax and provide migration guidance for legacy explicit `template(...)` usages.

Non-goals:
- Implementing exhaustive address data sources (continue using the existing list manager).
- Changing the runtime semantics of existing generators unless required for embedding support.

## High-level design

1. Parser/Grammar
-- Extend `dsl.pest` (or equivalent parser) to accept: `regexgen("<pattern>")` and dot notation for families (e.g., `address.city`, `name.firstname`).
-- Treat quoted strings that contain `{{...}}` as implicit templates, and accept concatenation expressions using `+` that mix strings and generator tokens.
-- Recognise named lists: `list("<name>")` (named list resource lookup).
-- Accept English shorthand `number between A and B` and canonicalise to the existing numeric AST shape.

2. AST changes
- Ensure `ast::Generator` enum includes `Regex(RegexGenerator)`, `Template(TemplateGenerator)`, `List(ListGenerator)` and `Address(AddressGenerator)` variants — these largely already exist; update fields if necessary (e.g., allow list names).

3. Factory and Generators
- `create_generator` already has variants for `Regex`, `Template`, `List`, and `Address`; wire parser outputs to those AST variants.
- Update `TemplateGenerator::validate` (already adjusted) to reject empty placeholders.

4. Docs & Migration
- Update `docs/DADAGEN_SYNTAX.md` with examples and canonical forms.
- Add migration guide to convert older Scala-like DSL strings into canonical DSL where needed.

5. Tooling
- Update `dadagen-gui` and `dadagen-jmeter` emission logic to output the new canonical forms where helpful.

## Example DSL snippets (canonical)

- Regex
  "uuid": regexgen("[a-f0-9]{8}-[a-f0-9]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}")

-- Template (preferred implicit form)
  "payload": "PERFT_{{id}}_{{r_uuid}}"
  
  (templates are expressed only via quoted strings with `{{...}}`; the `template(...)` surface form is removed. Parser tooling should provide a migration step to convert legacy `template(...)` occurrences into the implicit string form.)

- Address
  "town": address.city
  "postcode": address.postcode

-- Named list
  - Register or reference a named list resource using `list("<name-or-path>")`.
    Example: `list("lists/firstnames.csv")` or `list("firstnames")` (resolver will map names to configured list resources).
  - Use the named list in fields: `"firstname": list("firstnames")`.
  - Inline static enumerations: `enum("red","orange","yellow")` defines a small, static list available in-schema.

## Compatibility
- Keep support for old English-like shorthand by canonicalizing at parse time to existing AST shapes.
- Templates are expressed only via quoted strings with `{{...}}`; the explicit `template(...)` surface form is removed from the canonical DSL. Parser tooling should provide a migration step to convert legacy `template(...)` occurrences into the implicit string form.

## Nested templates and embedded generators (proposal)

Quoted strings with `{{...}}` become first-class implicit templates and can contain either field references or embedded generator expressions. Concatenation (`+`) composes strings and generators into a single composite generator; this lets authors hide `template(...)` behind plain strings and write natural expressions like `"foo {{other}}" + address.city`.

Key points:
- Quoted strings that include `{{...}}` are parsed as `TemplateLiteral` and converted to `Template` AST nodes.
- Placeholders may be:
  - plain field names: `{{id}}` → `FieldRef("id")`, or
  - generator expressions: `{{name.firstname}}` → `EmbeddedGenerator(Generator::Name(...))`.
- Unquoted generator tokens (e.g. `name.firstname`, `regexgen("...")`, `number between 1 and 10`) remain valid generator expressions.
- The `+` operator concatenates terms (strings, template literals, generator tokens) and is canonicalised into a single template/concat AST node.

Examples (preferred implicit forms):
- `"payload": "PERFT_{{id}}_{{r_uuid}}"`  -- implicit template
- `"display": "{{name.firstname}} {{name.surname}}"` -- embedded generator expressions
- `field1: name.firstname`
- `field2: "foo {{other_field}}" + address.city`

Parser notes:
- `STRING` tokens containing `{{` are parsed into `TemplatePart` sequences: literals and placeholders.
- Grammar supports `expr = term ("+" term)*` where `term` is `STRING | TemplateLiteral | GeneratorToken`.
- Placeholder content is parsed first as a generator expression (dot or function form) and, if that fails, as a plain field reference.

AST mapping and runtime behaviour:
- `TemplatePart` enum: `Literal(String) | FieldRef(String) | EmbeddedGenerator(Generator)`.
- `concat` expressions are canonicalised into `Generator::Template` (or `Generator::Concat`) containing `TemplatePart`s.
- During generation: literals are appended; `FieldRef` values are read from `Context`; `EmbeddedGenerator` parts are generated by invoking the nested generator. Dependencies include all `FieldRef` names plus nested generator dependencies.

Validation and edge-cases:
- Empty placeholders `{{}}` are rejected at parse/validation time.
- Support escaping `{{` and `}}` (recommend backslash-escape `\{\{` / `\}\}`) — document exact rules.
- Embedded generator failures may either bubble as generation errors or be rendered inline as `[error:spec]`; default behaviour is to return an error so issues are visible during data generation.



Next steps (implementation):
- Parser: extend `dsl.pest` to support `TemplateLiteral`, placeholder parsing, and `concat` expressions.
- AST: add `TemplatePart` and canonical `Template`/`Concat` representation.
- Factory: update `create_generator` so `Template`/`Concat` AST nodes produce a `TemplateDataGenerator` that evaluates parts and nested generators.
- Tests: add parsing and end-to-end tests covering embedded generators, concatenation and error cases.

## Risks
- Parser complexity: adding too many syntactic shorthands increases parsing ambiguity; prefer canonical AST outputs.
- Backwards compatibility with Scala DSL generators: provide migration examples.

## Acceptance Criteria (summary)
- Parser accepts and canonicalises the new forms.
- AST contains matching generator variants.
- `create_generator` returns the appropriate `DataGenerator` for each new form.
- Tests and docs updated with examples.
