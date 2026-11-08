---
tags:
  - spec
  - steps
---

# Steps

| | |
|---|---|
| **Trigger** | `flatcompose -f <file> build`, carrying a `steps` block |
| **Requires** | A FlatCompose file and its build, per [Compose](compose.md) |
| **Actor** | A user describing one FlatImage application as a file |

The `steps` block forms the assembly interface for FlatImage. The schema below defines the valid keys and their defaults.

```yaml
# [optional]
steps:
    # [required] 
  - builder:
      # [required]
      from: debian:bookworm
      # [optional, default = host architecture]
      arch: x86_64
      # [optional, default = 3]
      http_retries: 3
      # [required]
      install:
        # [required]
        patterns:
          - firefox-esr
        # [optional, default = package]
        type: package
        # [optional, default = any]
        match: any
        # [optional]
        with:
          - recommends
        # [optional]
        exclude:
          - some-package
        # [optional, default = ldconfig, scripts, hooks]
        postinstall:
          - ldconfig
        # [optional, default = 4]
        parallel: 4
        # [optional, default = false]
        no_deps: true
```

The `yaml` block above produces this command line:

```bash
fim-root builder \
    --from debian:bookworm \
    --arch x86_64 \
    --http-retries 3
    install \
        --type package \
        --match any \
        --with recommends \
        --exclude some-package \
        --postinstall ldconfig \
        --parallel 4 \
        --no-deps firefox-esr
```


## User Scenarios & Testing

### User Story: composing a root filesystem

As a user, I want to use the `builder` FlatImage command within the FlatCompose file, so that I can build rootfs systems pragmatically.

**Acceptance scenarios**:

1. **Given** a FlatCompose file with one `builder` step naming packages, **When** the user builds the FlatImage, **Then** the FlatImage contains a root filesystem, with those packages installed.
2. **Given** a FlatCompose file with two `builder` steps, **When** the user builds the FlatImage, **Then** both roots exist in the FlatImage, and each holds the packages of its own step.

## Requirements

### Step commands

**FR-STEPS-001**: the `steps` block MUST provide an interface for the `builder` command.
{: #fr-steps-001 }

**FR-STEPS-002**: the `steps` block MUST declare the commands FlatCompose runs against the FlatImage.
{: #fr-steps-002 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - ca-certificates
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
```

```text
fim-root builder --from debian:bookworm install ca-certificates
fim-root builder --from debian:bookworm install firefox-esr
```

**FR-STEPS-003**: FlatCompose MUST run the commands of `steps` in the order the entries appear in the file.
{: #fr-steps-003 }

### The root builder

**FR-BUILDER-001**: the FlatCompose file MUST define the `fim-root builder` command as `builder`.
{: #fr-builder-001 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
```

```text
fim-root builder --from debian:bookworm install firefox-esr
```

**FR-BUILDER-002**: the FlatCompose file MUST define the `--from <source>` option as `builder.from`, a required distribution source string such as `debian:bookworm`.
{: #fr-builder-002 }

```yaml
steps:
  - builder:
      from: ubuntu:noble@2024-06-15
      install:
        patterns:
          - firefox
```

```text
fim-root builder --from ubuntu:noble@2024-06-15 install firefox
```

**FR-BUILDER-003**: the FlatCompose file MUST define the `install` command as `builder.install`.
{: #fr-builder-003 }

**FR-BUILDER-004**: the FlatCompose file MUST define the `--arch <arch>` option as `builder.arch`, an architecture string such as `x86_64`.
{: #fr-builder-004 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      arch: aarch64
      install:
        patterns:
          - firefox-esr
```

```text
fim-root builder --from debian:bookworm --arch aarch64 install firefox-esr
```

**FR-BUILDER-005**: the FlatCompose file MUST define the `--http-retries <count>` option as `builder.http_retries`, an integer `<count>`.
{: #fr-builder-005 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      http_retries: 5
      install:
        patterns:
          - firefox-esr
```

```text
fim-root builder --from debian:bookworm --http-retries 5 install firefox-esr
```

**FR-BUILDER-006**: the FlatCompose file MUST define the patterns for the packages the builder installs as `builder.install.patterns`. FlatCompose MUST refuse a `builder` that omits it.
{: #fr-builder-006 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
          - ca-certificates
```

```text
fim-root builder --from debian:bookworm install firefox-esr ca-certificates
```

**FR-BUILDER-007**: the FlatCompose file MUST define the `--type <type>` option as `builder.install.type`, a pattern space string such as `package`.
{: #fr-builder-007 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - usr/bin/curl
        type: path
```

```text
fim-root builder --from debian:bookworm install --type path usr/bin/curl
```

**FR-BUILDER-008**: the FlatCompose file MUST define the `--match <match>` option as `builder.install.match`, a combination rule string such as `any`.
{: #fr-builder-008 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - usr/lib/libssl.so
          - usr/bin/curl
        type: path
        match: all
```

```text
fim-root builder --from debian:bookworm install --type path --match all usr/lib/libssl.so usr/bin/curl
```

**FR-BUILDER-009**: the FlatCompose file MUST define the `--with <kind>` option as `builder.install.with`, a list of optional dependency kind strings joined into one comma-separated argument.
{: #fr-builder-009 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
        with:
          - recommends
          - suggests
```

```text
fim-root builder --from debian:bookworm install --with recommends,suggests firefox-esr
```

**FR-BUILDER-010**: the FlatCompose file MUST define the `--exclude <package>` option as `builder.install.exclude`, a list of package name strings joined into one comma-separated argument.
{: #fr-builder-010 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
        exclude:
          - fonts-noto-color-emoji
```

```text
fim-root builder --from debian:bookworm install --exclude fonts-noto-color-emoji firefox-esr
```

**FR-BUILDER-011**: the FlatCompose file MUST define the `--postinstall <phase>` option as `builder.install.postinstall`, a list of post-install phase strings joined into one comma-separated argument.
{: #fr-builder-011 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
        postinstall:
          - scripts
          - hooks
```

```text
fim-root builder --from debian:bookworm install --postinstall scripts,hooks firefox-esr
```

**FR-BUILDER-012**: the FlatCompose file MUST define the `--parallel <count>` option as `builder.install.parallel`, an integer `<count>`.
{: #fr-builder-012 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
        parallel: 1
```

```text
fim-root builder --from debian:bookworm install --parallel 1 firefox-esr
```

**FR-BUILDER-013**: the FlatCompose file MUST define the `--no-deps` option as `builder.install.no_deps`, a boolean. `true` includes `--no-deps` in the command, and `false` omits it.
{: #fr-builder-013 }

```yaml
steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
        no_deps: true
```

```text
fim-root builder --from debian:bookworm install --no-deps firefox-esr
```

<!-- vale ai-tells.DoubleHyphen = NO -->
--8<-- "_glossary.md"
