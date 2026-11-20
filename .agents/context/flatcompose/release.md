# FlatCompose: release

**Read this when:** changing the release build or the CI workflows.

## The model

The release uses two Docker stages. `docker/Dockerfile.build.toolchain` starts from the cross-rs image for the target and adds rustup. `docker/Dockerfile.build.release` runs `cargo build --release --target` with `+crt-static` and `relocation-model = static`. It asserts zero `NEEDED` entries and checks for libnss and libresolv references. It copies the result to `/out/flatcompose-linux-<arch>`. `.github/workflows/release.yml` drives the five-architecture matrix and the release. `.github/workflows/test.yml` drives the test suite. The exact steps appear in those files.

## Contracts

- Each published binary links statically against musl with `+crt-static`. It runs directly on the Linux kernel.
- Release assets use the name `flatcompose-linux-<arch>` and include a `.sha256sum` file.
- A release tag takes the form `v<Cargo.toml version>` and must equal the HEAD of `origin/master`.
- The same workflow serves GitHub and Gitea, selected by `github.server_url`.
- The test job requires the `FLATIMAGE` repository variable.

## Gotchas

- The release Dockerfile fails on any `NEEDED` entry, including an unexpected libnss or libresolv reference.
- `relocation-model = static` keeps the output a plain static ELF rather than a static PIE.
- The test container needs `--privileged --device /dev/fuse`. Without it, each setting read fails.
- The repository ignores the test environment file. CI creates it from the repository variable, and `docker/.env.tests.example` documents its contents.

## Source map

| Question | File |
|---|---|
| The toolchain stage | `docker/Dockerfile.build.toolchain` |
| The release stage and static assertions | `docker/Dockerfile.build.release` |
| The release workflow | `.github/workflows/release.yml` |
| The test workflow | `.github/workflows/test.yml` |
| The GitHub/Gitea CLI setup | `.github/actions/setup-platform-cli/action.yml` |
| The release profile | `Cargo.toml` |
