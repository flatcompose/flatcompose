---
tags:
  - spec
  - image
---

# Image

| | |
|---|---|
| **Trigger** | `flatcompose -f <file> build` |
| **Requires** | A FlatCompose file; a FlatImage runtime file to build from, unless the output already exists |
| **Actor** | A user describing one FlatImage application as a file |

A FlatCompose file describes one FlatImage in one YAML file. It states the image it starts from and publishes, the build steps that mutate it, and the reserved-space configuration it declares. FlatCompose reads that file and turns each key into exactly one `fim-*` command, run against a copy of the file `from` names, and publishes the result. The `image` key gives both ends of the build; [Configure](configure.md) states the `config` block, and [Steps](steps.md) states the `steps` block.

FlatCompose accepts only the keys the schema declares. An unknown key ends the read, and the refusal gives the key it met.

Once the read succeeds, every violation the file holds comes out in one pass. A relative path resolves against the file's own directory.

## User Scenarios & Testing

### User Story 1: building an application

As a user, I want to build a FlatImage with a descriptive FlatCompose file, so that I can easily and pragmatically compose FlatImages.

**Acceptance scenarios**:

1. **Given** a FlatCompose file giving the runtime it starts from, an output path, and one install step, **When** the user builds, **Then** the build installs the packages into the root and publishes the artifact at the output path.
2. **Given** a published artifact, **When** the user builds again from the same FlatCompose file, **Then** the two artifacts show the same content.
3. **Given** a FlatCompose file with a key FlatCompose does not know, **When** the user builds, **Then** the command refuses, naming the key, and publishes nothing.
4. **Given** a FlatCompose file with three violations, **When** the user builds, **Then** the build reports all three in one run.
5. **Given** a FlatCompose file whose relative `from` gives a file beside it, **When** the user builds from another working directory, **Then** FlatCompose still finds the file.

## Requirements

### Image

A FlatCompose file builds exactly one FlatImage, and the `image` key gives both ends of that build: the file it starts from and the file it publishes. A build takes its input from that file alone and writes that artifact alone.

```yaml
image:
  from: core-x86_64.flatimage
  output: app.flatimage
  arch:
    - x86_64
```

Built from the directory containing both files, that FlatCompose file leaves the file it starts from as it found it and produces the artifact with its digest beside it:

```text
.
├── core-x86_64.flatimage          read by the build, unchanged
├── app.flatimage                  the published artifact
└── app.flatimage.sha256sum        the artifact's SHA-256 digest
```

**FR-IMAGE-001**: a FlatCompose file MUST have one top-level `image` key, with up to four keys that set the build:
{: #fr-image-001 }

- `from` gives the FlatImage the build starts from, and remains optional, per [FR-IMAGE-002](#fr-image-002).
- `output` gives the FlatImage the build publishes, and stays required, per [FR-IMAGE-003](#fr-image-003).
- `inplace` selects the file the build writes into, per [FR-IMAGE-004](#fr-image-004).
- `arch` lists the architectures every install step builds for, per [FR-IMAGE-005](#fr-image-005).

**FR-IMAGE-002**: `image.from` MUST give the FlatImage the build starts from, as a local file path or an `http(s)://` URL, and MUST remain optional. A relative path gives a file beside the FlatCompose file, so FlatCompose finds that file from any working directory. FlatCompose MUST work on a copy of that file and MUST leave the original unchanged. Without `from`, FlatCompose MUST start from the image already at `image.output`, so a FlatCompose file that adds to an image it already has omits the key.
{: #fr-image-002 }

```yaml
image:
  from: ./core-x86_64.flatimage
  output: app.flatimage
```

The build fetches an `http(s)://` URL:

```yaml
image:
  from: {{ FLATIMAGE_RELEASE_URL }}/core-x86_64.flatimage
  output: app.flatimage
```

Without `from`, the build starts from the file already at `output`:

```yaml
image:
  output: app.flatimage
```

**FR-IMAGE-003**: `image.output` MUST appear and give the file FlatCompose publishes. Beside it FlatCompose MUST write the artifact's SHA-256 digest as `<output>.sha256sum`, with the digest and the file name separated by two spaces, which `sha256sum -c` accepts.
{: #fr-image-003 }

```yaml
image:
  from: core-x86_64.flatimage
  output: app.flatimage
```

```console
$ flatcompose -f app.yml build

$ sha256sum -c app.flatimage.sha256sum
app.flatimage: OK
```

**FR-IMAGE-004**: `image.inplace` MUST select the file FlatCompose writes into.
{: #fr-image-004 }

`false`, the default, composes into a copy of the image it starts from and overwrites `image.output` with the result. A later build replaces the file that sits there.

```yaml
image:
  from: core-x86_64.flatimage
  output: app.flatimage
  inplace: false
```

```console
$ flatcompose -f app.yml build
```

The artifact stands as the FlatCompose file's own result: a second build of that file replaces `app.flatimage` with a fresh copy of `core-x86_64.flatimage` with the same steps.

`true` composes into the file already at `image.output` and keeps its content. The build then adds to the image already there.

```yaml
image:
  from: core-x86_64.flatimage
  output: app.flatimage
  inplace: true
```

```console
$ flatcompose -f app.yml build
```

A first build creates `app.flatimage` from `core-x86_64.flatimage`. Every later build adds to `app.flatimage` where it stands. While no file sits at `image.output`, both modes create it from `image.from`.

**FR-IMAGE-005**: `image.arch` MUST list the architectures every install step of the image builds for, as one or more of `x86_64`, `i686`, `aarch64`, `armv7l`, and `riscv64`. FlatCompose MUST pass the list to the package resolver as one comma-joined `--arch` argument on every install step. An absent list MUST leave the resolver on its host architecture.
{: #fr-image-005 }

```yaml
image:
  from: core-x86_64.flatimage
  output: app.flatimage
  arch:
    - x86_64
    - i686
```

The install step of that FlatCompose file passes to the resolver as:

```text
fim-root builder --from debian:bookworm --arch x86_64,i686 install firefox-esr
```

FlatCompose MUST refuse a token outside the five.

**FR-IMAGE-006**: FlatCompose MUST refuse a build when `image.from` stays absent and no file exists at `image.output`, because the build has nothing to work on.
{: #fr-image-006 }

```console
$ flatcompose -f app.yml build
error: `image.from` is absent and no file sits at `app.flatimage`
```

<!-- vale ai-tells.DoubleHyphen = NO -->
--8<-- "_glossary.md"
