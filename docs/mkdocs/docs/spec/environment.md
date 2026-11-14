---
tags:
  - spec
  - environment
---

# Environment

| | |
|---|---|
| **Trigger** | `flatcompose -f <file> build` |
| **Requires** | A FlatCompose file and its build, per [Image](image.md) |
| **Actor** | A user describing one FlatImage application as a file |

A FlatCompose file can take values from the environment. This page states the reference forms and every key that accepts one; [Image](image.md), [Configure](configure.md), and [Steps](steps.md) state each block's own keys. The schema below marks which of those keys accept a reference.

## User Scenarios & Testing

### User Story: interacting with the environment

As a user, I want to reference environment variables in a FlatCompose file, so that one file builds different FlatImages.

**Acceptance scenarios**:

1. **Given** a FlatCompose file whose values reference environment variables, **When** the user builds with those variables set, **Then** each reference takes the value the environment holds.

## Requirements

### Environment variables

**FR-ENV-VAR-001**: the FlatCompose file MUST accept `$NAME` or `${NAME}` in place of a value, and FlatCompose MUST use the value the build environment holds for `NAME`.
{: #fr-env-var-001 }

```yaml
config:
  env:
    EDITOR: $EDITOR
```

With `EDITOR=vim`:

```text
fim-env set EDITOR=vim
```

**FR-ENV-VAR-002**: FlatCompose MUST use the default of `${NAME:-default}` when the build environment leaves `NAME` unset or empty.
{: #fr-env-var-002 }

```yaml
config:
  env:
    GREETING: ${GREETING:-hello}
```

With `GREETING` unset or empty:

```text
fim-env set GREETING=hello
```

**FR-ENV-VAR-003**: FlatCompose MUST use the default of `${NAME-default}` when the build environment leaves `NAME` unset, and MUST keep an empty value empty.
{: #fr-env-var-003 }

```yaml
config:
  env:
    GREETING: ${GREETING-hello}
```

With `GREETING` set to an empty value:

```text
fim-env set GREETING=
```

**FR-ENV-VAR-004**: FlatCompose MUST substitute an empty value when a reference with no default refers to a variable the build environment leaves unset, and MUST print a warning on stderr with the variable's name.
{: #fr-env-var-004 }

```console
$ flatcompose -f app.yml build
warning: environment variable `EDITOR` is unset; substituting an empty value
```

**FR-ENV-VAR-005**: `$$` MUST write one `$`. A `$` before a variable name or `{` MUST open a reference, and the file MUST write every other `$` as `$$`.
{: #fr-env-var-005 }

```yaml
config:
  env:
    PATH: /opt/local/bin:$$PATH
```

```text
fim-env set PATH=/opt/local/bin:$PATH
```

```yaml
config:
  env:
    PRICE: $$5
```

```text
fim-env set PRICE=$5
```

**FR-ENV-VAR-006**: FlatCompose MUST resolve each reference once, and MUST NOT resolve a reference inside text that arrives from the build environment.
{: #fr-env-var-006 }

```yaml
config:
  env:
    GREETING: ${VALUE}
```

With `VALUE='${HOME}'`:

```text
fim-env set GREETING=${HOME}
```

**FR-ENV-VAR-007**: the FlatCompose file MUST accept a reference in every value it writes. The `exec.script` text reaches its shell as written, and FlatCompose MUST NOT resolve a reference in it.
{: #fr-env-var-007 }

The schema below lists every key the file writes and marks whether it accepts a reference. The variable names in the schema illustrate the form. Any name works in the build environment.

```yaml
image:
  # reference: the file or URL the build starts from
  from: ${CORE}
  # reference: the file the build publishes
  output: ${APP}
  # reference: true or false
  inplace: ${INPLACE}
  # reference: one architecture token per item
  arch:
    - ${ARCH}
steps:
  - builder:
      # reference: the distribution source
      from: ${SOURCE}
      # reference: one architecture token
      arch: ${ARCH}
      # reference: a retry count
      http_retries: ${RETRIES}
      install:
        # reference: one package or path per item
        patterns:
          - ${PACKAGE}
        # reference: package or path
        type: ${TYPE}
        # reference: any or all
        match: ${MATCH}
        # reference: one dependency kind per item
        with:
          - ${KIND}
        # reference: one package per item
        exclude:
          - ${EXCLUDE}
        # reference: one post-install phase per item
        postinstall:
          - ${PHASE}
        # reference: a count
        parallel: ${PARALLEL}
        # reference: true or false
        no_deps: ${NO_DEPS}
  - exec:
      # reference: the root the script runs in
      from: ${SOURCE}
      # reference: a user ID
      uid: ${UID}
      # reference: a group ID
      gid: ${GID}
      # no reference: the shell resolves its variables
      script: |
        echo $HOME
  - layer:
      overlay:
        save:
          # reference: the directory to compress
          from: ${ASSETS}
          # reference: the layer's destination
          to: ${LAYER}
      stack:
        add:
          # reference: the layer file
          path: ${LAYER}
          # reference: the mount path
          mount: ${MOUNT}
      mount:
        set:
          # reference: the layer hash
          hash: ${HASH}
          # reference: the mount path
          path: ${MOUNT}
        del:
          # reference: the layer hash
          hash: ${HASH}
        # no value
        clear: {}
config:
  root:
    # reference: the root name
    system: ${ROOT}
  # reference: one permission per item
  perms:
    - ${PERM}
  # reference: one option per item
  options:
    - ${OPTION}
  # reference: one namespace per item
  unshare:
    - ${NAMESPACE}
  # reference: one capsule per item
  capsule:
    - ${CAPSULE}
  # reference: the backend
  overlay: ${BACKEND}
  # reference: the mode
  sandbox: ${MODE}
  env:
    # reference: the name and the value
    ${NAME}: ${VALUE}
  boot:
    # reference: the program
    program: ${PROGRAM}
    # reference: one argument per item
    args:
      - ${ARG}
  # reference: src, dst, and mode per binding
  bind:
    - src: ${SRC}
      dst: ${DST}
      mode: ${MODE}
  desktop:
    # reference: the manifest path
    manifest: ${MANIFEST}
    # reference: the icon path
    icon: ${ICON}
    # reference: the entry path
    entry: ${ENTRY}
    # reference: the mimetype path
    mimetype: ${MIMETYPE}
    # reference: one integration per item
    integrations:
      - ${INTEGRATION}
```

<!-- vale ai-tells.DoubleHyphen = NO -->
--8<-- "_glossary.md"
