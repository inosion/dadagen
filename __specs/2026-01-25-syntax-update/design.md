# Syntax Update: Add Regex, Template, Address and Named Lists

## Overview

This spec focuses on the concrete set of changes required to make the new canonical DSL available end-to-end in the Rust codebase. The goal is to replace legacy surface forms with a small, consistent syntax family and implement it with minimal disruption.
There is no need to support any backwards compatibility as the new SYNTAX is breaking change.
We do not need to support old forms like `number between a and b` (canonicalised in the parser).

Primary change areas (summary):

-- Parser & Grammar
	- Support implicit templates: quoted strings containing `{{placeholder}}` (placeholders limited to field identifiers).
	- Support `+` concatenation to compose strings and generator expressions.
	- Recognise `list("name")` and `enum(...)` forms; retain existing generator functions such as `regexgen(...)`, `number(...)`.
	- Field declarations MUST use the colon (`:`) separator; the whitespace-only separator is NOT supported.

-- Fields
	- Support the hidden field forms. The following forms are supported (colon required):

```
"field_name": TypeExpr         # normal case
hidden : TypeExpr               # hidden field used as a dependency seed (locale, gender, country etc)
"field_name" hidden: TypeExpr  # field with hidden flag; useful when the value is needed in a template string (see example below)
```

	- When a field is declared `hidden` it participates in dependency resolution but is not emitted as an output column unless explicitly mapped.

- Linking
    - Support dependency linking using the existing resolver.
    - e.g. name depends on human.gender, or human.sex if either of these are used directly (`"field_gender": human.gender`), or implictly via hidden (`hidden : human.gender`)


-- Template semantics
	- Templates are implicit-only (no `template(...)` surface form).
	- Placeholders are identifiers only: content inside `{{...}}` MUST be a simple field name matching `[A-Za-z_][A-Za-z0-9_]*`.
	- Embedded generator expressions inside `{{...}}` are NOT permitted and MUST be rejected by the parser/validator.
	- Provide clear parse/validation errors for empty placeholders and escaping rules.
	- Example: with a hidden seed field declared as `"a_gender" hidden : gender`, another field may use `"salutation": "{{a_gender}} Mr/Ms"` to reference it.

### Generator Tokens (normative list)

Implementations MUST support the following generator families and tokens (this list is normative for the syntax update):

- `name` family: `name.firstname`, `name.surname`, `name.initial`, `name.title`
- `address` family: `address.city`, `address.postcode`, `address.street`, `address.suburb`, `address.district`, `address.country`, `address.state`, `address.housename`, `address.streetnumber`
- `locale([value,value,...]?)`
- `number(min,max)` — integer range (exclusive upper bound by default)
- `float(min,max, precision=N)`
- `enum("A","B","C")`
- `list("name")` or `list("path/to/file.txt")` with optional `mode` parameter (`random` default, `sequential` optional)
- `sequence(start, end?, step?)`
- `regexgen("<pattern>")`
- `human.*` family: `human.eyecolor`, `human.haircolor`, `human.height`, `human.weight`, `human.age`, `human.dob`, `human.sex`, `human.gender`
- `contact.*` family: `contact.email`, `contact.phone`, `contact.mobile`
- `date.*` family: `date.time`, `date.date`, `date.full`, `date.format(...)`, `date.now`
- `uuid.*` family: `uuid.v4`
- `hash.*` family: `hash.sha256`, `hash.blake3`

Notes: the parser should recognise generator identifiers and delegate argument parsing to generator-specific handlers; adding new generator families should be straightforward.

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

