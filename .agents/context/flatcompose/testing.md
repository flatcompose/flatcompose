# FlatCompose: testing

**Read this when:** adding or fixing a test, or changing the harness.

## The model

Two tiers.

- The source file for a module contains its unit test under `#[cfg(test)]`. The unit test checks the words and order of one command's arguments. It has a `// covers:` tag that specifies the spec item.
- An integration test appears under `tests/`. It runs the built binary and reads the result through the runtime's own verbs. It checks the published file and the settings the runtime reports from it.

The harness gives each test its own directory and fills `image.from` from `FLATIMAGE`. It runs `flatcompose -f <file> build` there. `tests/README.md` maps each `// covers:` tag to a row. `tests/AGENTS.md` states the decisions a test cannot overrule.

The nextest profile in `.config/nextest.toml` runs four test threads. It never fails fast. It retries three times with backoff and warns past a 360-second test.

## Contracts

The test contract appears in `tests/README.md` and `tests/AGENTS.md`. Those pages state the test rules and the coverage bijection. The code adds two facts. The harness asserts `/.dockerenv`. It reads `FLATIMAGE`.

## Gotchas

- Reading a setting back executes the real image. The container needs `/dev/fuse`.
- A failed build can leave `image.output` partly configured. Cases assert the digest and the error message.
- FlatCompose fetches a URL in `FLATIMAGE` into each test's directory. A URL run needs room for one copy per test. The harness uses a local path as given.

## Source map

| Question | File |
|---|---|
| The harness and outcome checks | `tests/harness/common.rs` |
| The runtime wrapper the cases read through | `tests/harness/flatimage.rs` |
| The coverage table | `tests/README.md` |
| The decisions the suite follows | `tests/AGENTS.md` |
| The nextest profile | `.config/nextest.toml` |
| The container test recipe | `docker/Dockerfile.tests` |
