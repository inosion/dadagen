# Dadagen DSL — Syntax Reference

This document specifies the canonical Dadagen schema DSL. It is language-agnostic and designed to be easy to parse, human-readable, and simple to merge/override.

Summary
-------
- Field declarations map a quoted name to a generator expression.
- Use a colon `:` as the canonical delimiter, whitespace-only form is accepted for convenience.
- Generator families support dot-subtypes: `address.city`, `name.firstname`.
- Numeric bounds are exclusive by default; use square-brackets for inclusive ranges.
- `list(...)`, `enum(...)`, and `sequence(...)` are supported for arrays/enums/sequences.

Top-level
---------
Two equivalent forms are accepted:

- Anonymous schema:

```
schema {
  "name": name.firstname
  "age": number(18,65)
}
```

- Named schema (name is informational; both forms are equivalent):

```
schema People {
  "name": name.firstname
  "age": number(18,65)
}
```

Field declaration
-----------------
Syntax:

```
"field_name": TypeExpr
```

Both of these are accepted (parser normalises to the colon form):

```
"town" address.city
"town": address.city
```

Type expressions
----------------
General form:

```
generator[params]
generator(args)
generator.subtype(args)
```

Common generators and forms:

- `name`, `name.firstname`, `name.surname`
- `address.city`, `address.postcode`
- `string`, `string(pattern="<regex>", min=..., max=...)`
- `number(min,max)` (exclusive upper bound by default)
- `number[min,max]` (inclusive bounds)
- `float(min,max, precision=N)` (precision optional)
- `enum("A","B","C")` or `list("a","b","c")` for discrete choices
- `list(type, n)` fixed-length list
- `list(type, min..max)` ranged-length list (inclusive bounds)
- `sequence(start, end?, step?)` — sequence of numbers; loops by default if `end` provided (use `loop=false` to stop)

Template interpolation
----------------------
Templates use `{{...}}` interpolation for field placeholders. Use `{{field_name}}` to insert a previously-generated field value into a template string. Example:

```
"fullname": template("{{firstname}} {{surname}}")
```

Escaping: use `\{{` to emit a literal `{{` in output. Template placeholders accept only simple identifiers (no arbitrary expressions) in the MVP.

Notes on semantics
------------------
- Number bounds: `number(1,40)` → integers >=1 and <40 (upper exclusive). Use `number[1,40]` for inclusive 1..40.
- Dot notation selects a subtype/generator family: `name.firstname` picks the firstname generator from the `name` family.
- `enum(...)` and `list(...)` may accept strings or numbers. `list` is also used for repeated values.
- `sequence` behaviour: with `sequence(1,10)` the generator emits 1..10 then loops back to 1. To produce a non-looping sequence, set `loop=false`.

Override & merge rules
----------------------
- Override schemas can be provided with a subset of fields. Merging rules:
  - If override contains a field present in default → replace the generator for that field.
  - If override contains a new field → append it to the schema.
  - Unmentioned fields keep their default generators.

Examples
--------

People schema (canonical):

```
schema People {
  "id": rownumber
  "r_uuid": regexgen("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
  "gender": gender
  "firstname": name.firstname
  "surname": name.surname
  "fullname": template("{{firstname}} {{surname}}")
  "dob": regexgen("19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])")
  "street_number": number(1,100)
  "street_name": template("RS Performance Street")
  "town": address.city
  "postcode": regexgen("[A-Z][A-Z][0-9] [0-9][A-Z][A-Z]")
  "initial_investment": number(10000,90000) precision 2
  "retirement_age": number(65,75)
}
```

Override example (only age changed):

```
schema {
  "age": number(1,40)
}
```

Notes for implementers
----------------------
- Parser should accept both colon and whitespace forms but normalise to colon form in AST/pretty-printing.
- Support english shorthand (`number between 1 and 40`) as a syntactic sugar that canonicalises to `number(1,40)`.
- Bounds are inclusive/exclusive as specified above; make that explicit in parser docs and tests.
- Provide a small linter/pretty-printer to convert legacy `field { "name" <gen> }` forms to the canonical form.

Comments
--------
Supported comment styles:

- Line comments starting with `#` (preferred for DSL/config users):

```
# This is a comment
```

- Line comments starting with `//` (familiar to programmers):

```
// This is also a comment
```

- Block comments using `/* ... */` (allow multilines):

```
/*
  Multi-line comment
  spanning several lines
*/
```

Parser notes:
- Both `#` and `//` are treated as line comments (ignored from the marker to end-of-line).
- `/* ... */` is treated as a block comment and can span multiple lines.
- Comment markers inside quoted strings are not treated as comments.


Appendix: quick reference
------------------------
- Field: `"name": generator`  
- Subtype: `generator.subtype`  
- Number (exclusive): `number(a,b)`  
- Number (inclusive): `number[a,b]`  
- Enum: `enum("a","b","c")`  
- List fixed: `list(type, n)`  
- List range: `list(type, min..max)`  
- Sequence: `sequence(start, end?, step?, loop=true)`
