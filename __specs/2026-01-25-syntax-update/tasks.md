# Implementation Tasks: DSL Syntax Update

This tasks list implements the requirements in `requirements.md`. Tasks are ordered and include acceptance criteria and rough estimates.

- [ ] # 1 Parser changes — enforce colon-only fields, templates, placeholders, concat, list/enum parsing
- Estimate: M (6-12h)
- Steps:
  - Update `dsl.pest` or parser module to accept only `"field": expr` forms; reject whitespace-only field separators.
  - Add rules for `TemplateLiteral` parsing (split quoted string into `Literal` and `{{placeholder}}` segments).
  - Add `placeholder` rule that accepts identifiers and dotted paths (e.g. `prop.foo`) per the regex `[A-Za-z_][A-Za-z0-9_.-]*`; ensure empty placeholders fail.
  - Add tests verifying dotted placeholders are parsed and later resolved as nested field references.
  - Add `concat`/`plus` rule for `term ("+" term)*` and ensure function calls bind tighter than `+`.
  - Add parsing for `list("name")` and `enum("a","b")` forms.
  - Add unit tests that cover valid and invalid field declarations, templates, placeholders, concatenation precedence, and list/enum parsing.
- Acceptance:
  - New parser unit tests pass; whitespace-field syntax rejected by parser errors.

- [ ] # 2 AST updates — `TemplatePart`, `Template`/`Concat` node and hidden field flag
- Estimate: S (2-4h)
- Steps:
  - Add `TemplatePart` enum: `Literal(String)` and `FieldRef(String)`.
  - Add `Generator::Template { parts: Vec<TemplatePart> }` or a canonical `Concat` node.
  - Add `hidden` flag on field AST nodes and ensure AST produces it for `hidden : TypeExpr` and `"field" hidden: TypeExpr` forms.
  - Add AST roundtrip tests for hidden fields and templates.
- Acceptance:
  - AST tests pass; hidden flag appears where expected.

- [ ] # 3 Factory mapping — `create_generator` → `TemplateDataGenerator`, list/enum factories
- Estimate: M (4-8h)
- Steps:
  - Update `create_generator` to construct `TemplateDataGenerator` instances from `Template` AST nodes.
  - Ensure `TemplateDataGenerator` captures dependency set from `FieldRef` parts.
  - Map `list` and `enum` AST nodes to `ListDataGenerator`/`ChoiceGenerator` factory branches.
  - Add unit tests verifying factory outputs and dependency reporting.
- Acceptance:
  - Factory unit tests pass and dependencies are correct.

- [ ] # 4 Runtime — implement `TemplateDataGenerator` behaviour
- Estimate: M (6-12h)
- Steps:
  - Implement generation: iterate `TemplatePart`s, resolve `FieldRef` values from `context`, append literals.
  - Define and implement strict vs tolerant behaviour for missing referenced fields (strict = error, tolerant = inline marker).
  - Ensure proper error types (`GenerationError`) on runtime failures.
  - Add unit tests for substitution, missing-field behaviour, and dependency-ordering generation.
- Acceptance:
  - Generator unit tests pass; integration test with a small schema demonstrates correct ordering and substitution.

- [ ] # 5 List resolution & `ListDataGenerator`
- Estimate: M (6-12h)
- Steps:
  - Implement resolver lookup: path detection (contains `/` or `./`), registry lookup (config/list_data), and error messages.
  - Support lazy or eager loading per engine conventions; document chosen approach.
  - Add tests for list loading, missing list error messages, and mode parameter (`random` default).
- Acceptance:
  - List resolution tests pass; missing lists produce actionable errors.

- [ ] # 6 Enum generator implementation
- Estimate: S (2-4h)
- Steps:
  - Implement `EnumGenerator` as a small choice generator.
  - Add tests for selection semantics and optional sequential mode.
- Acceptance:
  - Enum tests pass.

- [ ] # 7 Validation, escaping and docs updates
- Estimate: S (3-6h)
- Steps:
  - Implement validation for empty/invalid placeholders during parse/validate.
  - Implement `\{{` escaping in parser → literal segment.
  - Update `docs/DADAGEN_SYNTAX.md` with canonical examples and hidden-field forms.
  - Add docs examples to `config/` as sample schemas.
- Acceptance:
  - Validation tests and docs update are present and correct.

- [ ] # 8 Migration tooling (optional)
- Estimate: M (1-2d)
- Steps:
  - Implement a CLI tool to rewrite legacy shorthand forms to canonical forms where possible.
  - Add tests for common legacy patterns.
- Acceptance:
  - Tool rewrites supported patterns safely and tests pass.

- [ ] # 9 CI and integration
- Estimate: S (2-4h)
- Steps:
  - Add new tests to CI and run full suite.
  - Address any integration failures or lints.
- Acceptance:
  - CI passes on feature branch.


## Recommended order
1 → 2 → 3 → 4 → 5/6 → 7 → 8 → 9

Stop and run unit tests after each numbered task. After tasks complete, run integration & CI.
