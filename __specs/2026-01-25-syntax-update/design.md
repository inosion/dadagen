# Syntax Update: Add Regex, Template, Address and Named Lists

## Overview

This spec focuses on the concrete set of changes required to make the new canonical DSL available end-to-end in the Rust codebase. The goal is to replace legacy surface forms with a small, consistent syntax family and implement it with minimal disruption.
There is no need to support any backwards compatibility as the new SYNTAX is breaking change.
We do not need to support old forms like `number between a and b` (canonicalised in the parser).

Primary change areas (summary):

- Parser & grammar
	- Support implicit templates: quoted strings containing `{{placeholder}}` (placeholders limited to field identifiers).
	- Support `+` concatenation to compose strings and generator expressions.
	- Recognise `list("name")` and `enum(...)` forms; retain existing generator functions such as `regexgen(...)`, `number(...)`
    - Commas separate fields

- Fields
    - Support the hidden field `hidden : TypeExpr`

- Linking
    - Support dependency linking using the existing resolver.
    - e.g. name depends on human.gender, or human.sex if either of these are used directly (`"field_gender": human.gender`), or implictly via hidden (`hidden : human.gender`)

- Template semantics
	- Templates are implicit-only (no `template(...)` surface form).
	- Placeholders are identifiers only (no nested generator expressions inside `{{...}}`).
	- Provide clear parse/validation errors for empty placeholders and escaping rules.

- AST & factory
	- Add `Template`/`TemplatePart` AST nodes for literal segments and field placeholders.
	- Canonicalise concat expressions into a single AST node that `create_generator` maps to a `TemplateDataGenerator` at runtime.

- Named lists & enums
	- Standardise on `list("<name-or-path>")` for named lists; implement resolver lookup and allow file paths.
	- Support `enum(...)` for small inline lists.
    - Both support mode=random (default) or mode=sequential

- New Generators present in the old Scala
    - Dates, Human properties, Contact

- Runtime wiring and list resolver
	- Wire `ListDataGenerator` to the list resolver (config + bundled lists) and provide sensible fallbacks.

- Tests, docs and tooling
	- Add parser unit tests, AST roundtrip tests, generator factory tests and end-to-end generation tests (templates, concat, lists, enums).
	- Update sample configs and provide a lightweight migration tool or linter to rewrite legacy `template(...)` and `listFrom(...)` usages.
	- Update emitters (GUI, JMeter) to produce canonical DSL.

