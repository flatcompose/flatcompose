# The decisions the test suite rests on

This file states the decisions a change to one test cannot overrule. A case that needs a different rule states the change here first.

## The suite runs in a container

Every test writes under `$HOME/flatcompose/tests`. The suite runs inside the container `docker/Dockerfile.tests` builds, with `/dev/fuse` and `fuse3` present. `harness::require_container` refuses to start a case outside a container. A case reads a setting by executing flatimage, and that execution mounts through FUSE.

The commands to run:

- `cp docker/.env.tests.example docker/.env.tests`, then fill `FLATIMAGE` in it
- `docker build -f docker/Dockerfile.tests -t flatcompose-tests .`
- `docker run --rm --privileged --device /dev/fuse --env-file docker/.env.tests flatcompose-tests`

`docker/.env.tests` stays untracked, because the address of the flatimage it names belongs to the machine it runs on. The sample beside it lists every name the suite reads and states what to fill.

## The test runs the built binary

A test runs the product from `env!("CARGO_BIN_EXE_flatcompose")`, which cargo fills with the binary it built for this package. A test never calls a source function instead of the binary, and never reads a library path the product would not.

## The base image comes from one variable

`FLATIMAGE` names the image the build starts from, and `docker/.env.tests` states it. `Harness::new` reads it once and puts it in `image.from`. The harness takes a path as given. It fetches a URL into the test's own directory first. Either way, `image.from` points at a local path.

A missing value fails the test with a message that states the variable. It never skips the test.

## One directory per test

Each test works in a directory of its own, and the path it gets has one part per bullet:

- `$HOME` gives the home directory of whoever runs the suite.
- `flatcompose/tests` gives the parent directory of every test.
- `<label>` states the word the case hands to `Harness::new`, describing what the case checks.
- `<pid>` gives the process id of the test run.
- `<serial>` gives the next number of a counter. Cases of one process never share a directory.

The directory goes when the test ends.

Every file a test creates stands inside that directory, and every file the suite creates stands there too. A test writes inside its own directory, never to `/tmp`, to the home directory itself, or to the source tree.

## Reading a setting

A test never parses the image file. It executes flatimage at `image.output` and reads what that execution prints, using the commands `tests/README.md` lists.

A test says what a user sees, and a change to how flatimage stores a setting leaves the test untouched.

## A case fills the types, never YAML text

A case builds the FlatCompose file as the product's own types, and `Harness::write` turns them into the file. No case assembles YAML text by hand.

A case writes raw text for a key or a value the types cannot represent.

## The tag states the rule

Each test function states a `// covers: <id>` tag naming the spec item it checks, and `tests/README.md` lists the row that matches it. The tags and the rows cover the same set. A read of these files together finds any gap.

A case exists for a rule a spec page states. When a case checks a behaviour the spec does not state, the spec page changes first.

## Each test target declares the harness

Cargo makes one test target of each `tests/<block>/main.rs`. The directory `tests/harness/` omits `main.rs`, which keeps it out of that discovery and gives the harness one copy. Each target declares the harness files its cases use, by path:

```rust
#[path = "../harness/common.rs"]
mod common;

#[path = "../harness/flatimage.rs"]
mod flatimage;

#[path = "../harness/mock/mod.rs"]
mod mock;
```

`harness/flatimage.rs` wraps flatimage's own command model, cut to the command families the spec reads. Copy a family whole when a case needs one, so a reader can compare it with flatimage's own CLI.

## Comparing a value in the form flatimage keeps

flatimage stores a key as a set or reports its entries sorted by name. A case compares a list in the form flatimage keeps, and never asserts the order the FlatCompose file wrote.

A case reads a refusal as `outcome.refused_with(needle)`. Where a refusal happens before the digest step, the case also checks that no digest file stands beside the output.

## The spec writes the tests

A case states a rule a spec page states, and a case comes from that page. A case can exist before the code it checks. The spec alone defines what a case checks.

When a case fails, the code has the bug. Nobody rewrites a case to match the code it caught.

A case changes when the spec changes, in the same change as the spec.

## A test never turns itself off

No `#[ignore]`, no `cfg` that removes a case, and no return that skips a check. A case that cannot run fails with a message stating what it lacks, which matches how a missing `FLATIMAGE` behaves.
