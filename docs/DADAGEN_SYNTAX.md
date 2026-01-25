# Dadagen DSL — Syntax Reference

This document specifies the canonical Dadagen schema DSL. It is language-agnostic and designed to be easy to parse, human-readable, and simple to merge/override.

## Summary

- Field declarations map a quoted name to a generator expression.
- Use a colon `:` as the canonical delimiter, whitespace-only form is accepted for convenience.
- Use the bare word `hidden` for a field you need as a calculator, for dependency but is not mapped to the output
  - `hidden : locale, ...`
- Generator families support dot-subtypes: `address.city`, `name.firstname`.
- Numeric bounds are exclusive by default; use square-brackets for inclusive ranges.
- `list(...)`, `enum(...)`, and `sequence(...)` are supported for arrays/enums/sequences.

## Top-level

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

## Field declaration

Syntax:

```
"field_name": TypeExpr
     hidden : TypeExpr
```

## Type expressions

General form:

```
generator[params]
generator(args)
generator.subtype(args)
```

## Complete set of Generators

- `"raw string"` - with templates for other fields 
   - "{{field1}}-ABC"
   - "{{firstname}}.{{lastname}}@" + list("potatoes") + ".com
- `name` - `name.firstname`, `name.surname`, `name.initial`, `name.title`
- `address.city`, `address.postcode`, `address.street`, `address.suburb`, `address.city`, `address.district`, `address.country`, `address.state`, `address.housename`, `address.streetnumber`
- `locale([value,value,...]?)` - Locale
- `number(min,max)` (exclusive upper bound by default)
- `float(min,max, precision=N)` (precision optional)
- `enum("A","B","C")`
- `list("rocks", mode?)`, `list("potatoes")`, `list("./mycustom-list.txt")` is a predefined list, or a user supplied list which accepts a filename.
  - `mode` can be sequential, or random
- `sequence(start, end?, step?)` — sequence of numbers; loops by default if `end` provided
- `regexgen("[a-z][a-z0-9]+@[a-z]{4,6}[0-9]+\.com")`
- `sequence(start?)` - incrementing number - start is the beginning value. `start=0` is the default.
- `human` - `human.eyecolor`, `human.haircolor`, `human.height`, `human.weight`, `human.age`, `human.dob`, `human.sex`, `human.gender`
- `contact` - `contact.email`, `contact.phone`, `contact.mobile`
- `date` - `date.time`, `date.date`, `date.full`, `date.format(...)`, `date.now`
- `uuid.*` - `uuid.v4`
- `hash` - `hash.sha256`, `hash.blake3`


### Linked Properties

- The following properties are linked (the value of one is dependent upon the value of another). To not use the dependency, supply `no-depends` e.g. 
- locale based 
  - address.*
  - name.*
  - human.*
  - contact.*
- human.sex
  - name.*
  - human.*
- human.gender
  - name.*
  - human.*


## Template interpolation

Templates are found in the strings. `{{...}}` interpolation for field placeholders. Use `{{field_name}}` to insert a previously-generated field value into a template string. Example:

```
"fullname": "{{firstname}} {{surname}}"
```

Escaping: use `\{{` to emit a literal `{{` in output. Template placeholders accept only simple identifiers (no arbitrary expressions) in the MVP.

## Notes on semantics

- Number bounds: `number(1,40)` → integers >=1 and <40 (upper exclusive). Use `number[1,40]` for inclusive 1..40.
- Dot notation selects a subtype/generator family: `name.firstname` picks the firstname generator from the `name` family.
- `enum(...)` and `list(...)` may accept strings or numbers. `list` is also used for repeated values.
- `sequence` behaviour: with `sequence(1,10)` the generator emits 1..10 then loops back to 1. To produce a non-looping sequence, set `loop=false`.

## Nested fields

As the schema supports generating JSON, nesting fields looks like 

```
{ 
   "prop" : {
      "foo" : name.firstname,
      "bar" : number(1,100)
   },
   "somefield" : list["a","b","c"]
   
}
```

When this is generated to CSV, it will produce `prop.foo, prop.bar, somefield` rows.

## Override & merge rules

When used in programs as test generation, the dadgen inspector will automatically make a schema, from the struct, obect, class that you supply. 
In these situations, a field may want to be altered from __dadagen's__ guess. 

- Override schemas can be provided with a subset of fields. Merging rules:
  - If override contains a field present in default → replace the generator for that field.
  - If override contains a new field → append it to the schema.
  - Unmentioned fields keep their default generators.

## Examples


People schema (canonical):

```
schema People {
  "id": sequence
  "r_uuid": regexgen("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
  "gender": gender
  "firstname": name.firstname
  "surname": name.surname
  "fullname": "{{firstname}} {{surname}}"
  "dob": regexgen("19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])")
  "street_number": number(1,100)
  "street_name": "RS " +  number(100,900) + " " + address.street_type
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

## Notes for implementers

- Parser should accept both colon and whitespace forms but normalise to colon form in AST/pretty-printing.
- Bounds are inclusive/exclusive as specified above; make that explicit in parser docs and tests.

## Comments

Supported comment styles:
  - `//`, `#` and `/* ... */`

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

## Appendix: quick reference

- Field: `"name": generator`
- Subtype: `generator.subtype`
- Integer number (exclusive upper bound): `number(a,b)`
- Integer number (inclusive bounds): `number[a,b]`
- Floating number: `float(min,max, precision=N)`
- Enum (inline static list): `enum("a","b","c")`
- Named list resource: `list("name")` or `list("path/to/file.txt")` — use in fields as `"field": list("name")`
- Sequence: `sequence(start=0, end?, step?, loop=true)`
- Regex generator: `regexgen("<pattern>")`
- Mix and match quoted string with `{{placeholder}}` interpolation, e.g. `"fullname": "{{firstname}} {{surname}}"`
- Concatenate expressions with `+` to mix strings and generators: `"prefix-" + name.firstname + "-" + regexgen("[0-9]{4}")`
- Address / name families: `address.city`, `address.postcode`, `name.firstname`, `name.surname`

Notes:
- Templates are expressed only via quoted strings containing `{{...}}` placeholders.
- Placeholder content may be a field name (`{{id}}`) or an embedded generator expression (`{{name.firstname}}`). Embedded generators are evaluated and substituted.
- Escape `{{` as `\{{` to emit a literal `{{` in output.
- Use `list("...")` to reference named lists managed by the list resolver; `enum(...)` is for small inline lists defined in-schema.
