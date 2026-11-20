<a id="top"></a>

<h1 align="center">FlatCompose</h1>

<p align="center">
  Declarative composition for FlatImages
</p>

--- 

[Quickstart](#quickstart) · [Related work](#related-work) · [License](#license)

---

[FlatImage](https://github.com/flatimage/flatimage) provides a portable Linux application format. Building one by hand means running command-line commands in sequence, from choosing a distribution root, installing packages, configuring, committing layers and so on. This project aims to simplify and standardize this process with FlatCompose files, which declare the full building and configuration steps in one file.

## Quickstart

[Back to top](#top)

Download the static FlatCompose binary for your architecture:

```bash
wget -O flatcompose https://github.com/flatcompose/flatcompose/releases/latest/download/flatcompose-linux-$(uname -m)
chmod +x flatcompose
```

Write a `flatcompose.yml`:

```yaml
image:
  from: https://github.com/flatimage/flatimage/releases/latest/download/core-x86_64.flatimage
  output: firefox.flatimage

steps:
  - builder:
      from: debian:bookworm
      install:
        patterns:
          - firefox-esr
  - exec:
      from: debian:bookworm
      script: |
        mkdir -p /home/firefox/.mozilla
  - layer:
      overlay:
        save: {}

config:
  root:
    system: debian-bookworm
  perms:
    - network
    - xorg
    - wayland
    - audio
    - gpu
  capsule:
    - gpu
    - audio
  env:
    HOME: /home/firefox
    USER: firefox
    LOGNAME: firefox
    XDG_CONFIG_HOME: /home/firefox/.config
    XDG_DATA_HOME: /home/firefox/.local/share
    XDG_STATE_HOME: /home/firefox/.local/state
    XDG_CACHE_HOME: /home/firefox/.cache
  boot:
    program: /usr/bin/firefox-esr
```

Next, on the same directory you wrote `flatcompose.yml`, run `flatcompose` to build the image:

```bash
./flatcompose -f flatcompose.yml build
```

FlatCompose then creates `firefox.flatimage`. Now you can try out the application with:

```bash
./firefox.flatimage
```

## Related work

[Back to top](#top)

FlatCompose produces a FlatImage runtime. It builds that artifact by translating a flatcompose file into the runtime's own commands. Here follows the inspiration for the design of the flatcompose files.

**Container build files.** FlatCompose takes inspiration from [Dockerfile](https://docs.docker.com/reference/dockerfile/) on the build model: a build starts from a base image and applies changes to it one after another. Each change works on the state the previous one left, and the final state becomes the image. Dockerfile writes those changes as an ordered text file of instructions, and one command reads the file and produces the image. FlatCompose does the same. Its file specifies the image the build starts from and lists the changes in the order they run. Each change applies one operation to the image, and the next change works on the result. FlatCompose sets the environment and startup command during the build and stores them in the result, matching Dockerfile's environment and startup settings. One command reads the file and produces the FlatImage.

**Multi-container composition.** FlatCompose takes inspiration from [Docker Compose](https://docs.docker.com/compose/) on the layout of the file: one YAML file with a top-level key for each part of the application. Compose uses it for services, networks, and volumes. FlatCompose uses the same layout for a build, with a top-level key for each part of the build.

**CI workflow files and reusable presets.** FlatCompose takes inspiration from [GitHub Actions](https://docs.github.com/en/actions/) on the list of steps that drives a workflow, where each step specifies one operation together with the values that operation needs. FlatCompose uses that format for its build: each one specifies one operation and supplies its values, in the order the file writes them. Its steps build one image.

## License

[Back to top](#top)

Apache License 2.0
