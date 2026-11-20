# FlatCompose: environment

**Read this when:** changing interpolation or escaping.

## The model

`Environment` reads the process environment once. `Resolve` walks the parsed YAML value tree and resolves each string and each mapping key. `Exclusion` specifies the paths the walk leaves alone.

The walk uses `Xpanda` in lenient mode. A strict pass reports an unset variable. The resolver warns once per name before substituting an empty string.

## Contracts

The accepted forms and the unset behavior appear in `docs/mkdocs/docs/spec/environment.md` (`FR-ENV-VAR-001` through `FR-ENV-VAR-007`). The code adds the structural exclusion and the scalar readers for resolved text. It also resolves mapping keys.

## Gotchas

- The exclusion path matches structurally. It applies to the `exec.script` field at any step index.
- FlatCompose warns once per variable per file.

## Source map

| Question | File |
|---|---|
| The environment and the exclusion | `src/internal/environment.rs` |
| Scalar readers for resolved text | `src/internal/scalar.rs` |
| Where the exclusion appears | `src/compose/compose.rs` |
| The user-facing grammar | `docs/mkdocs/docs/spec/environment.md` |
