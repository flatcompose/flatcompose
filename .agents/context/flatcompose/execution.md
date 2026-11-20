# FlatCompose: execution

**Read this when:** changing the build sequence or its failure behavior.

## The model

`Compose::build` runs four moves.

1. `prepare` puts `image.output` in place. With `inplace: true`, the build uses the existing output as it stands. Otherwise the build copies a local `from` or fetches a remote `from`. With no `from`, the build uses the output as it stands. A copied or fetched output receives mode `0755`.
2. `steps.lower()` returns the step commands. Each runs in order.
3. `config.lower()` returns the config commands. Each runs in order.
4. `digest` writes the SHA-256 of the output to `<output>.sha256sum` as `<digest>  <file-name>` with a newline.

`run` resolves the image path to an absolute path and executes the image with the command's argv. It stops the build when the exit status differs from zero.

The `cli::build` entry point parses the file and collects each violation. It prints them in one pass. When the violation set empties, it calls `build`.

The artifact contract these moves follow appears in `docs/mkdocs/docs/spec/image.md` (`FR-IMAGE-002` through `FR-IMAGE-004`). The step and config order appears in `docs/mkdocs/docs/spec/steps.md` (`FR-STEPS-003`) and `docs/mkdocs/docs/spec/configure.md` (`FR-CONFIG-002`).

## Contracts

Commands run one at a time against `image.output`. The first failure ends the build, and later commands do not run. The CLI has one verb, `build`.

## Gotchas

- A failed command leaves `image.output` partly configured. FlatCompose leaves the partial state in place.
- With `inplace: true`, the partly configured file becomes the existing artifact.
- `prepare` marks a copied or fetched output executable. An output kept under `inplace: true` skips the copy step.

## Source map

| Question | File |
|---|---|
| The build sequence, `prepare`, `run`, and `digest` | `src/compose/compose.rs` |
| The CLI entry point and violation report | `src/cli/build.rs` |
| The one-command argv representation | `src/lower/command.rs` |
| The digest helper | `src/internal/hash.rs` |
| The remote fetch helper | `src/internal/http.rs` |
