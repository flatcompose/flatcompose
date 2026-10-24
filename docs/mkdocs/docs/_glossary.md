<!--
  Maintainer note: each term is repeated for every grammatical inflection
  it appears in (singular/plural, sometimes adjective forms) because the
  mkdocs `*[term]:` abbreviation extension matches exact strings, not
  lemmas. Adding a new term means adding every form that appears in the
  prose. Removing a duplicate "for cleanup" silently breaks the tooltip
  for that form.

  This is the single unified glossary for the entire FlatCompose
  documentation. Every page includes it via `--8<-- "_glossary.md"`.

  Terms whose meaning shifts with context are intentionally absent, because
  defining them once would mislead a reader who meets them in another
  surrounding.
-->

<!-- vale ai-tells.ColonUsage = NO -->
<!-- Each line is an abbreviation definition, `*[term]: Definition`, whose colon opens a sentence rather than a clause, so the capital after it breaks no rule. -->

<!-- A -->

*[active root]: The root a launch resolves the image's `/` from, stored in the reserved space and written with `fim-root system use`. With no value, the merge root itself becomes `/`.
*[architecture]: A CPU family named as `uname -m` reports it: `x86_64`, `i686`, `aarch64`, `armv7l`, `riscv64`. `image.arch` lists the ones every install builds for.
*[architectures]: CPU families named as `uname -m` reports them.
*[artifact]: The FlatImage file a build publishes at `image.output`, with its `<output>.sha256sum` sidecar beside it.
*[artifacts]: The FlatImage files builds publish.

<!-- B -->

*[boot program]: The program and arguments a launch starts, stored by `fim-boot set` from `config.boot`.
*[build]: One run of `flatcompose` that reads a FlatCompose file and publishes the artifact it describes.
*[builds]: Runs of `flatcompose` that read one FlatCompose file and publish its artifact.

<!-- C -->

*[capsule]: FlatImage's injection of the host's GPU or audio driver userspace into the sandbox at launch; `config.capsule` persists the set.
*[capsules]: FlatImage's host driver injections, GPU and audio.
*[checksum]: The SHA-256 digest written beside the artifact as `<output>.sha256sum`: the digest, two spaces, and the artifact's file name on one line.
*[committed layer]: A layer `fim-layer overlay save` appended, recorded in the ledger as `committed`. It takes no mount.
*[committed layers]: Layers `fim-layer overlay save` appended and recorded as `committed`.
*[config block]: The `config` key of a FlatCompose file, one value per stored reserved-space field, applied after every step.
*[context key]: A step-level key that renders around the operation's command rather than inside it: `from`, `bind`, `env`, `perms`, `capsule`, `unshare`, and `options`.
*[context keys]: Step-level keys that render around the operation's command.

<!-- D -->

*[desktop integration]: The application-menu presence `config.desktop` declares, applied with `fim-desktop`.

<!-- vale ai-tells.MotionMetaphors = NO -->
<!-- `DwarFS` reads as the verb "dwarfs" to the motion rule, and the name recurs from this entry onward, so the exemption runs to the end of the file. -->
*[DwarFS]: Deduplicating Warp-speed Advanced Read-only File System, the compressed read-only filesystem every FlatImage layer uses.

<!-- E -->

*[ephemeral]: Applied for one command and undone after it, never part of the artifact. Every step grant remains ephemeral.
*[exec step]: A step whose operation runs a shell script through `fim-exec`.
*[exec steps]: Steps whose operation runs a shell script through `fim-exec`.

<!-- F -->

*[FlatCompose]: The declarative builder that reads a FlatCompose file and turns each key into exactly one `fim-*` command.
*[FlatCompose file]: The YAML document, passed with `-f`, that describes one FlatImage: its `image` block, its `layer` entries, its `steps`, and its `config`.
*[FlatCompose files]: The YAML documents that describe one FlatImage each.
*[FlatImage]: The single-file Linux application FlatCompose builds: one executable made of the runtime, the appended layers and the reserved space.
*[FlatImages]: The single-file Linux applications FlatCompose builds.
*[FlatRoot]: The rootfs builder FlatImage embeds as `fim-root builder`. An install step maps onto its grammar one to one.
*[FUSE]: Filesystem in Userspace, the kernel interface FlatImage mounts its DwarFS layers through.

<!-- I -->

*[image block]: The `image` key of a FlatCompose file: the FlatImage the build starts from (`from`), the file it publishes (`output`), whether it composes in place, and the architectures installs build for.
*[included layer]: A layer `fim-layer stack add` appended from a layer file, recorded in the ledger as `included`. Only an included layer takes a mount.
*[included layers]: Layers `fim-layer stack add` appended from layer files.
*[inplace]: The `image.inplace` choice. `true` composes into the file already at `output`; `false`, the default, composes into a copy and overwrites `output`.
*[install step]: A step whose operation installs packages into a root through `fim-root builder`.
*[install steps]: Steps whose operation installs packages through `fim-root builder`.
*[interpolation]: The pass that expands `$` forms in a FlatCompose file before the build reads it.

<!-- L -->

*[layer]: A compressed DwarFS image appended to a FlatImage, named in the top-level `layer` key in append order.
*[layers]: Compressed DwarFS images appended to a FlatImage.
*[layer entry]: One item of `layer`, keyed by the `fim-layer` noun that produces it: `stack`, `overlay`, or `mount`.
*[layer entries]: The items of `layer`, each keyed by a `fim-layer` noun.
*[layer file]: A `.layer` file on disk with one DwarFS image, appended with `fim-layer stack add`.
*[layer files]: `.layer` files on disk, each with one DwarFS image.
*[layer ledger]: The image's own record of appended layers, one entry per frame with its hash, offset, size, kind, and destination.

<!-- M -->

*[layer mount]: The `mount` layer entry: a mountpoint recorded for a layer by its content hash with `fim-layer mount set`.
*[layer mounts]: The `mount` layer entries recording mountpoints by content hash.

<!-- O -->

*[output]: The `image.output` file a build publishes. Required in every FlatCompose file.
*[overlay backend]: The mechanism FlatImage merges layers with: `overlayfs`, `unionfs`, or `builtin`, set by `config.overlay`.
*[overlay layer]: The `overlay` layer entry: a directory compressed with `fim-layer overlay save` and appended as a committed layer, or written to a file when `to` names one.
*[overlay layers]: The `overlay` layer entries, compressed directories appended or written to a file.

<!-- P -->

*[permissions]: The host resources a launch receives, such as `home`, `network`, and `gpu`, persisted by `config.perms`.
*[publish]: Writing the composed file to `image.output` and its digest to `<output>.sha256sum`.

<!-- R -->

*[reserved space]: The writable region inside a FlatImage holding its configuration, one slot per stored field.
*[root filesystem]: The tree a launch resolves its `/` from, installed by `fim-root builder` into a named root.

<!-- S -->

*[sandbox]: The bubblewrap launch that confines the image's program, selected by `config.sandbox`.
*[sidecar]: The `<output>.sha256sum` file written beside the artifact.
*[source image]: The FlatImage `image.from` names. A build starts from it when no file sits at `output`.
*[stack entry]: The `stack` layer entry: a layer file appended with `fim-layer stack add`, with an optional mount.
*[stack entries]: The `stack` layer entries: layer files appended with `fim-layer stack add`.
*[step]: One operation of a FlatCompose file together with the context that operation accepts.
*[steps]: The operations of a FlatCompose file, each with the context its operation accepts.

<!-- U -->

*[unshare]: The namespaces a launch creates for itself, persisted by `config.unshare`.

<!-- W -->

*[working file]: The file a build composes into before publishing. It starts as a copy of the source image, or the file at `output` itself when the build runs in place.

<!-- vale ai-tells.MotionMetaphors = YES -->
