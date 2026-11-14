---
tags:
  - spec
  - configure
---

# Configure

| | |
|---|---|
| **Trigger** | `flatcompose -f <file> build`, carrying a `config` block |
| **Requires** | A FlatCompose file and its build, per [Image](image.md) |
| **Actor** | A user describing one FlatImage application as a file |

The `config` block configures the FlatImage: the permissions, namespaces, runtime options, overlay backend, sandbox mode, and so on. FlatCompose turns each key into the `fim-*` commands it states and runs them once the build's steps have run. The FlatImage then ends the build in the state the block declares.

The schema lists every key with the command it writes:

```yaml
# [optional]
config:
  # [optional] fim-root system use debian-bookworm
  root:
    # [required]
    system: debian-bookworm
  # [optional] fim-perms set home network gpu
  perms:
    - home
    - network
    - gpu
  # [optional] fim-options set casefold
  options:
    - casefold
  # [optional] fim-unshare set pid net
  unshare:
    - pid
    - net
  # [optional] fim-capsule set gpu audio
  capsule:
    - gpu
    - audio
  # [optional] fim-overlay set overlayfs
  overlay: overlayfs
  # [optional] fim-sandbox set bwrap
  sandbox: bwrap
  # [optional] fim-env set LANG=C.UTF-8
  env:
    LANG: C.UTF-8
  # [optional] fim-boot set /usr/bin/app --fullscreen
  boot:
    # [required]
    program: /usr/bin/app
    # [optional]
    args:
      - --fullscreen
  # [optional] fim-bind set /tmp:/host_tmp:rw
  bind:
    # [required]
    - src: /tmp
      # [required]
      dst: /host_tmp
      # [required]
      mode: rw
  # [optional]
  desktop:
    # [optional] fim-desktop manifest set manifest.json
    manifest: manifest.json
    # [optional] fim-desktop icon set icon.png
    icon: icon.png
    # [optional] fim-desktop entry set entry.desktop
    entry: entry.desktop
    # [optional] fim-desktop mimetype set mimetype.xml
    mimetype: mimetype.xml
    # [optional] fim-desktop integrations set entry icon
    integrations:
      - entry
      - icon
```

## User Scenarios & Testing

### User Story: a configured image

A user writes their application's configuration into the FlatCompose file once. Every later build keeps that configuration in the FlatImage.

**Acceptance scenarios**:

1. **Given** a FlatCompose file that states a configuration, **When** the user builds the FlatImage, **Then** the built FlatImage shows that configuration.
2. **Given** a user who changes an entry of the configuration in the FlatCompose file, **When** they build the FlatImage again, **Then** the built FlatImage shows the entry they now wrote.
3. **Given** a user who built the FlatImage from a FlatCompose file and left the file unchanged, **When** they build it again, **Then** the built FlatImage shows the same configuration as the first build.

## Requirements

### The reserved-space configuration

**FR-CONFIG-001**: the `config` block declares the FlatImage's reserved-space configuration, and each key in it MUST come from the schema this page states.
{: #fr-config-001 }

**FR-CONFIG-002**: FlatCompose MUST run the `fim-*` command of every key in the file's `config` block, once the `steps` finish.
{: #fr-config-002 }

**FR-CONFIG-003**: FlatCompose MUST pass every value of the `config` block through unchanged. Whether a value passes validation stays the runtime's decision.
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

**FR-OPTIONS-001**: `config.options` names the runtime toggles the FlatImage uses. FlatCompose MUST write them as one `fim-options set` command.
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

**FR-ENV-001**: `config.env` names the environment variables the FlatImage sets, as a mapping of variable name to value. FlatCompose MUST write the mapping as one `fim-env set NAME=VALUE` command, one argument per variable, in the order the FlatCompose file defines them.
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

**FR-BOOT-001**: `config.boot` names the startup command the FlatImage runs, as the required `program` and the optional `args` list. FlatCompose MUST write it as `fim-boot set <program> [args...]`, the program first and the arguments in the order the file writes them.
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

**FR-BIND-001**: `config.bind` names the path bindings, each one as `src`, `dst`, and `mode`, all required. FlatCompose MUST write each binding as one `src:dst:mode` argument of one `fim-bind set` command.
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

**FR-DESKTOP-001**: `config.desktop` names the desktop assets the FlatImage contains, one field per asset:
{: #fr-desktop-001 }

- `manifest` names the manifest JSON, which gives the application's name, its categories, and the items to integrate.
- `icon` names the image drawn beside the application.
- `entry` names a `.desktop` file.
- `mimetype` names a MIME package file.
- `integrations` name the integration items to enable.

FlatCompose MUST write each of the four paths as `fim-desktop <field> set <path>`, and the `integrations` items as one `fim-desktop integrations set <item>...` command.

Entries name paths to regular files, either full or relative to the working directory FlatCompose runs from.

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

**FR-ROOT-001**: the boot root sets the root the FlatImage executes in, and `config.root.system` MUST give that root's name. FlatCompose MUST write the name as `fim-root system use <root>`.
{: #fr-root-001 }

```yaml
config:
  root:
    system: debian-bookworm
```

```text
fim-root system use debian-bookworm
```

<!-- vale ai-tells.DoubleHyphen = NO -->
--8<-- "_glossary.md"
