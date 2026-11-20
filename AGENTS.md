# FlatCompose

FlatCompose reads one `flatcompose.yml` and validates it. It then prepares the image it will publish and turns each written key into the FlatImage runtime's own command. Each command runs against the image. The build publishes the artifact beside a SHA-256 digest. The project provides a Rust CLI with one verb, `build`. FlatCompose runs the FlatImage runtime as a separate process and passes that process an argument list.

## Layout

- `src/cli/`: argv parsing and the build entry point.
- `src/compose/`: the parsed file and its entities, `Compose`, `image/`, `steps/`, `config/`.
- `src/lower/`: `Command`, the argv of one `fim-*` command, and `Lower`, the trait that turns one entity into its commands.
- `src/internal/`: environment interpolation, SHA-256, HTTP fetch, and scalar readers.
- `src/violation.rs`: `Violation`, `Entry`, and `Violations`.
- `tests/`: the integration tests and the harness. `tests/README.md` and `tests/AGENTS.md` state the contract.
- `docs/mkdocs/docs/spec/`: the current user-facing contract.
- `docker/`: the build, test, and documentation recipes.
- `.github/`: the release and test workflows and the platform-CLI action.

## Toolchain and build

Rust edition 2024. A local build runs `cargo build --release`. The release recipe runs `docker/Dockerfile.build.toolchain`, then `docker/Dockerfile.build.release`. The release profile appears in `Cargo.toml`. The static-linkage assertions and the output path appear in the release Dockerfile. The formatting rules appear in `rustfmt.toml`.

## Verification

The unit tests for a module appear beside it under `#[cfg(test)]`. Integration tests appear under `tests/`. The test contract and the coverage table appear in `tests/README.md` and `tests/AGENTS.md`. The container recipe appears in `docker/Dockerfile.tests`, and CI calls it from `.github/workflows/test.yml`. The suite needs `FLATIMAGE`. `docker/.env.tests.example` documents the file and the variable. It also needs a privileged container with `/dev/fuse`. The exact commands of the pre-commit hook appear in `.githooks/pre-commit`.

## Hard invariants

1. Each emitted command translates a key the file wrote. The build adds what the file declares.
2. The build leaves the file that `image.from` specifies unchanged (`FR-IMAGE-002`). The base image provides the input.
3. Unknown keys end the read, and the error reports the key (`FR-IMAGE-001`). The strict-read paragraph in `docs/mkdocs/docs/spec/image.md` states the rule. `deny_unknown_fields` marks each block.
4. FlatCompose collects the rule violations and prints them in one pass before any command runs (`docs/mkdocs/docs/spec/image.md`, introduction and user story 1, scenario 4).
5. The steps run in file order (`FR-STEPS-003`), and the config block runs after them (`FR-CONFIG-002`).
6. FlatCompose leaves `exec.script` untouched (`FR-ENV-VAR-007`). The shell resolves its variables.
7. An unset reference resolves to an empty string when the file omits a default, and FlatCompose warns once per variable (`FR-ENV-VAR-004`).
8. Each entity renders its own argv. FlatCompose builds commands in each entity.
9. FlatCompose refuses a layer key with zero commands or more than one (`FR-LAYER-013`).
10. The FlatImage runtime runs as a separate process. FlatCompose passes that process an argument list.
11. The `exec` script runs with errexit and pipefail (`FR-EXEC-004`).

## Code style

- Fallible functions return `anyhow::Result`. `.with_context` reports the path or command that failed.
- The naming pattern uses `entity_action_qualifier` for free functions, `action_qualifier` for methods, `type_qualifier` for variables, and `path_filetype_qualifier` for paths. A module-scoped function omits the module name.
- Serde types store the parsed data and refuse unknown fields. Validation and lowering appear in the entity file that defines the key.
- Doc comments state the role and the contract. Inline comments state why. The `// covers:` tag above a unit test specifies the spec item it checks.

## Failure modes

- An install step reads `builder.arch`. FlatCompose validates `image.arch` without forwarding it.
- A failed build leaves `image.output` partly configured. With `inplace: true`, it mutates the file already there.

## Topics

A task reads the matching topic page before touching its subsystem.

| Working on… | Read |
|---|---|
| The top-level file or the `image` block | [grammar.md](.agents/context/flatcompose/grammar.md) |
| Steps | [steps.md](.agents/context/flatcompose/steps.md) |
| The `config` block and its keys | [config.md](.agents/context/flatcompose/config.md) |
| Environment references and escaping | [environment.md](.agents/context/flatcompose/environment.md) |
| The build sequence and its failures | [execution.md](.agents/context/flatcompose/execution.md) |
| Tests and the container suite | [testing.md](.agents/context/flatcompose/testing.md) |
| The static build and releases | [release.md](.agents/context/flatcompose/release.md) |

## Where to look

| Question | Authoritative source |
|---|---|
| The literal YAML structure | the serde types under `src/compose/` |
| What a key means, and its validation | the entity's `lower` and `violations` under `src/compose/` |
| What commands a file produces | each entity's `lower()` and `src/lower/command.rs` |
| Build order and output preparation | `src/compose/compose.rs` and `src/cli/build.rs` |
| Interpolation and its exclusions | `src/internal/environment.rs` |
| The user-facing grammar | `docs/mkdocs/docs/spec/` |
| Test coverage and test rules | `tests/README.md` and `tests/AGENTS.md` |
| The container test command | `docker/Dockerfile.tests` and `.github/workflows/test.yml` |
| Static linkage and release assets | `docker/Dockerfile.build.release` and `.github/workflows/release.yml` |
| Lint and format rules | `.vale.ini`, `rustfmt.toml`, and `.githooks/pre-commit` |
