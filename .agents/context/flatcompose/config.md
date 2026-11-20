# FlatCompose: config

**Read this when:** adding or changing a `config` key or the command it emits.

## The model

`Config` provides one optional entity per key the schema lists. Each entity renders its own `lower` and `violations`. `Config` calls the keys it contains and gathers what they return. The keys and their meanings appear in `docs/mkdocs/docs/spec/configure.md`. The exact set appears in `src/compose/config/config.rs`.

`Config::lower` walks its keys in the order the code fixes. That order differs from the schema page. The order appears in `src/compose/config/config.rs`.

Values pass through unchanged. The runtime validates each value.

## Contracts

The keys and their commands appear in `docs/mkdocs/docs/spec/configure.md`. The code adds two facts. The config block runs after the steps. Each key renders its own argv.

## Gotchas

- The lowering order places `root` last, whatever order the file uses. A file that writes `root` above `perms` still emits `fim-perms set ...` before `fim-root system use ...`.
- One `desktop` key can emit up to five commands: one each for the manifest, icon, entry, and mimetype, plus one for the integrations. A file that states all five pieces produces five `fim-desktop` commands.
- An invalid value fails when its own command runs, after earlier steps have changed the output. A `builder` step installs its packages first. Then `perms: [nonsense]` emits `fim-perms set nonsense`, the runtime rejects it, and `image.output` still contains the installed packages.
- The runtime stores `perms`, `options`, `unshare`, and `capsule` as sets. It reports them sorted by name rather than in file order. A file that writes `perms: [network, home]` reads back `home` before `network`.

## Source map

| Question | File |
|---|---|
| The `config` block and its lowering order | `src/compose/config/config.rs` |
| One key's entity and command | `src/compose/config/<key>.rs` |
| The user-facing grammar | `docs/mkdocs/docs/spec/configure.md` |
