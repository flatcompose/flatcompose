# FlatCompose: steps

**Read this when:** adding or changing a `builder`, `exec`, or `layer` step, its fields, or its lowered command.

## The model

`Steps` provides a list of `Step`. A `Step` accepts an optional `builder`, `exec`, and `layer`. `Step::lower` runs them in that order. The deserializer records each `exec` step's position, which specifies its script file. The step grammar and each operation's fields appear in `docs/mkdocs/docs/spec/steps.md` (`FR-STEPS-*`, `FR-BUILDER-*`, `FR-EXEC-*`, `FR-LAYER-*`). The lowerings appear in `src/compose/steps/`.

## Contracts

The spec fixes each operation's fields and refusals. The code adds two facts. A step may state more than one operation. FlatCompose writes the `exec` script during lowering.

## Gotchas

- FlatCompose writes the `exec` script during lowering, before the build runs.
- `builder.arch` provides the per-step target. FlatCompose validates `image.arch` without forwarding it.
- A step may state more than one of `builder`, `exec`, and `layer`. A step runs its stated operations in a fixed order.

## Source map

| Question | File |
|---|---|
| The `steps` list and entry order | `src/compose/steps/steps.rs` |
| The `builder` command | `src/compose/steps/builder.rs` |
| The `exec` command and script path | `src/compose/steps/exec.rs` |
| The `layer` commands and their count | `src/compose/steps/layer.rs` |
| The user-facing grammar | `docs/mkdocs/docs/spec/steps.md` |
