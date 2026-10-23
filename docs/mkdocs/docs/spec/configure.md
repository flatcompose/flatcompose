---
tags:
  - spec
  - configure
---

# Configure

| | |
|---|---|
| **Trigger** | `flatcompose -f <file> build`, carrying a `config` block |
| **Requires** | A FlatCompose file and its build, per [Compose](compose.md) |
| **Actor** | A user describing one FlatImage application as a file |

The `config` block configures the FlatImage: the permissions, namespaces, runtime options, overlay backend, sandbox mode, and so on. FlatCompose turns each key into the `fim-*` commands it states and runs them once the build's steps have run, so the FlatImage ends the build in the state the block declares.

The schema carries every key, each with the command it writes:

```yaml
config:
  # fim-root system use debian-bookworm
  root:
    system: debian-bookworm
  # fim-perms set home network gpu
  perms:
    - home
    - network
    - gpu
  # fim-options set casefold
  options:
    - casefold
  # fim-unshare set pid net
  unshare:
    - pid
    - net
  # fim-capsule set gpu audio
  capsule:
    - gpu
    - audio
  # fim-overlay set overlayfs
  overlay: overlayfs
  # fim-sandbox set bwrap
  sandbox: bwrap
  # fim-env set LANG=C.UTF-8
  env:
    LANG: C.UTF-8
  # fim-boot set /usr/bin/app --fullscreen
  boot:
    program: /usr/bin/app
    args:
      - --fullscreen
  # fim-bind set /tmp:/host_tmp:rw
  bind:
    - src: /tmp
      dst: /host_tmp
      mode: rw
  # fim-desktop manifest set manifest.json
  # fim-desktop icon set icon.png
  # fim-desktop entry set entry.desktop
  # fim-desktop mimetype set mimetype.xml
  # fim-desktop integrations set entry icon
  desktop:
    manifest: manifest.json
    icon: icon.png
    entry: entry.desktop
    mimetype: mimetype.xml
    integrations:
      - entry
      - icon
```

## User Scenarios & Testing

### User Story: shipping a configured image

A user writes their application's configuration into the FlatCompose file once, so the FlatImage ships with it.

**Acceptance scenarios**:

1. **Given** a FlatCompose file that carries a configuration, **When** the user builds the FlatImage, **Then** the FlatImage ships with that configuration.
2. **Given** a user who changes an entry of the configuration in the FlatCompose file, **When** they build the FlatImage again, **Then** the FlatImage ships with the entry they now wrote.
3. **Given** a user who built the FlatImage from a FlatCompose file and left the file unchanged, **When** they build it again, **Then** the FlatImage ships with the same configuration as the first build.

## Requirements

### The reserved-space configuration

**FR-CONFIG-001**: The `config` block is the FlatImage's reserved-space configuration, and every key it carries MUST be one of the keys the [Compose schema](compose.md#schema) declares.
{: #fr-config-001 }

**FR-CONFIG-002**: FlatCompose MUST run the `fim-*` command of every key the file's `config` block carries, once the `steps` are finished.
{: #fr-config-002 }

**FR-CONFIG-003**: FlatCompose MUST pass every value of the `config` block through unchanged. Whether a value is valid is the runtime's decision.
{: #fr-config-003 }

```yaml
config:
  perms:
    - home
    - network
```

```text
fim-perms set home network
```

### Sandbox permissions

**FR-PERMS-001**: `config.perms` names the host resources the sandbox grants the FlatImage. FlatCompose MUST write them as one `fim-perms set` command, in the order the file writes them.
{: #fr-perms-001 }

```yaml
config:
  perms:
    - network
    - home
    - gpu
```

```text
fim-perms set network home gpu
```

### Host driver capsules

**FR-CAPSULE-001**: `config.capsule` names the host driver stacks whose userspace libraries the FlatImage links into its container on every execution. FlatCompose MUST write them as one `fim-capsule set` command.
{: #fr-capsule-001 }

```yaml
config:
  capsule:
    - gpu
    - audio
```

```text
fim-capsule set gpu audio
```

### Kernel namespaces

**FR-UNSHARE-001**: `config.unshare` names the kernel namespaces the sandbox detaches before the FlatImage executes. FlatCompose MUST write them as one `fim-unshare set` command.
{: #fr-unshare-001 }

```yaml
config:
  unshare:
    - pid
    - net
```

```text
fim-unshare set pid net
```

### Runtime options

**FR-OPTIONS-001**: `config.options` names the runtime toggles the FlatImage carries. FlatCompose MUST write them as one `fim-options set` command.
{: #fr-options-001 }

```yaml
config:
  options:
    - casefold
```

```text
fim-options set casefold
```

### Overlay backend

**FR-OVERLAY-001**: `config.overlay` names the overlay backend the FlatImage uses. FlatCompose MUST write it as `fim-overlay set <backend>`.
{: #fr-overlay-001 }

```yaml
config:
  overlay: overlayfs
```

```text
fim-overlay set overlayfs
```

### Sandbox mode

**FR-SANDBOX-001**: `config.sandbox` names the mode the FlatImage executes in. FlatCompose MUST write it as `fim-sandbox set <mode>`.
{: #fr-sandbox-001 }

```yaml
config:
  sandbox: bwrap
```

```text
fim-sandbox set bwrap
```

### Environment variables

**FR-ENV-001**: `config.env` names the environment variables the FlatImage carries, as a mapping of variable name to value. FlatCompose MUST write the mapping as one `fim-env set NAME=VALUE` command, one argument per variable, in the order they're defined on FlatCompose.
{: #fr-env-001 }

```yaml
config:
  env:
    LANG: C.UTF-8
    EDITOR: vi
```

```text
fim-env set LANG=C.UTF-8 EDITOR=vi
```

### Startup command

**FR-BOOT-001**: `config.boot` names the startup command the FlatImage carries, as the required `program` and the optional `args` list. FlatCompose MUST write it as `fim-boot set <program> [args...]`, the program first and the arguments in the order the file writes them.
{: #fr-boot-001 }

```yaml
config:
  boot:
    program: /usr/bin/app
    args:
      - --fullscreen
      - --profile=main
```

```text
fim-boot set /usr/bin/app --fullscreen --profile=main
```

### Host binds

**FR-BIND-001**: `config.bind` names the path bindings, each one as `src`, `dst`, and `mode`, all required. FlatCompose MUST write each binding as one `src:dst:mode` argument of a single `fim-bind set` command.
{: #fr-bind-001 }

```yaml
config:
  bind:
    - src: /tmp
      dst: /host_tmp
      mode: rw
    - src: /etc/localtime
      dst: /etc/localtime
      mode: ro
```

```text
fim-bind set /tmp:/host_tmp:rw /etc/localtime:/etc/localtime:ro
```

### Desktop integration

**FR-DESKTOP-001**: `config.desktop` names the desktop assets the FlatImage ships, one field per asset:
{: #fr-desktop-001 }

- `manifest` is the manifest JSON, which carries the application's name, its categories, and the items to integrate.
- `icon` is the image drawn beside the application.
- `entry` is a `.desktop` file.
- `mimetype` is a MIME package file.
- `integrations` are the integration items to enable.

FlatCompose MUST write each of the four paths as `fim-desktop <field> set <path>`, and the `integrations` items as one `fim-desktop integrations set <item>...` command.

Entries are paths to regular files, either full or relative to the working directory FlatCompose is called from.

```yaml
config:
  desktop:
    manifest: manifest.json
```

```text
fim-desktop manifest set manifest.json
```

```yaml
config:
  desktop:
    icon: icon.png
```

```text
fim-desktop icon set icon.png
```

```yaml
config:
  desktop:
    entry: entry.desktop
```

```text
fim-desktop entry set entry.desktop
```

```yaml
config:
  desktop:
    mimetype: mimetype.xml
```

```text
fim-desktop mimetype set mimetype.xml
```

```yaml
config:
  desktop:
    integrations:
      - entry
      - icon
```

```text
fim-desktop integrations set entry icon
```

### The boot root

**FR-ROOT-001**: The boot root is the root the FlatImage executes in, and `config.root.system` MUST name it. FlatCompose MUST write the name as `fim-root system use <root>`.
{: #fr-root-001 }

```yaml
config:
  root:
    system: debian-bookworm
```

```text
fim-root system use debian-bookworm
```

--8<-- "_glossary.md"
