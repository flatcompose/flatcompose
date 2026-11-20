# FlatCompose: grammar

**Read this when:** changing the top-level file or the `image` block.

## The model

`Compose` provides the whole file: a required `image`, and optional `steps` and `config`. The `image` block gives the two ends of the build, `from` and `output`, and the optional `inplace` and `arch` keys.

`Compose::parse` reads the file as text and parses it to a YAML value. It resolves environment references across that value and deserializes the result into the entity types. Each block refuses unknown fields. A key outside the schema, or a value the type refuses, ends the read, and the error reports what it met.

A file that reads cleanly then gets a semantic check. The read and the check work separately: serde refuses a broken structure, and `violations()` refuses a value that read cleanly.

## Contracts

The keys, their defaults, and their refusals appear in `docs/mkdocs/docs/spec/image.md` (`FR-IMAGE-001` through `FR-IMAGE-006`). The code adds one fact the spec omits: serde refuses a broken structure, and `violations()` refuses a value that read cleanly.

## Gotchas

- `image.arch` receives validation without forwarding. An install step reads `builder.arch`.
- Interpolation runs before deserialization. A reference can supply a key as well as a value.
- A file that fails the read stops before the violation pass. A read failure and a violation failure produce different messages.

## Source map

| Question | File |
|---|---|
| The literal top-level structure | `src/compose/compose.rs` |
| The `image` keys and their types | `src/compose/image/image.rs` |
| The user-facing grammar | `docs/mkdocs/docs/spec/image.md` |
