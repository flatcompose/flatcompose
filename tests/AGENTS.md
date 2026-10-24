# The decisions the test suite rests on

This file states the decisions a change to one test cannot overrule. A case that needs a different rule states the change here first.

## The suite runs in a container

Every test writes under `$HOME/flatcompose/tests`, so the suite runs inside the container `docker/Dockerfile.tests` builds, with `/dev/fuse` and `fuse3` present. `harness::require_container` refuses to start a case outside a container, because reading a setting means executing flatimage, and that execution mounts through FUSE.

The commands are:

- `cp docker/.env.tests.example docker/.env.tests`, then fill `FLATIMAGE` in it
- `docker build -f docker/Dockerfile.tests -t flatcompose-tests .`
- `docker run --rm --privileged --device /dev/fuse --env-file docker/.env.tests flatcompose-tests`

`docker/.env.tests` stays untracked, because the address of the flatimage it names belongs to the machine it runs on. The sample beside it holds every name the suite reads and states what to fill.

## The product is the built binary

A test reaches the product as `env!("CARGO_BIN_EXE_flatcompose")`, which cargo fills with the binary it built for this package. No test calls a source function to stand in for the binary, and no test reads a library path the product would not.

## The base image comes from one variable

`FLATIMAGE` names the image the build starts from, and `docker/.env.tests` states it. `Harness::new` reads it once and puts it in `image.from`: a path is used as given, and a URL is fetched into the test's own directory first, so `image.from` always holds a local path.

A missing value fails the test with a message naming the variable. It never skips the test.

## One directory per test

Each test works in a directory of its own, and the path it gets has one part per bullet:

- `$HOME` is the home directory of whoever runs the suite.
- `flatcompose/tests` is where the harness keeps every test's directory.
- `<label>` is the word the case hands to `Harness::new`, naming what the case checks.
- `<pid>` is the process id of the test run.
- `<serial>` is the next number of a counter, so cases of one process never share a directory.

The directory goes when the test ends.

Every file a test creates stands inside that directory, and nothing the suite creates stands outside it. No test writes to `/tmp`, to the home directory itself, or to the directory the source tree lives in.

## How a test reads a setting

A test never parses the image file. It executes flatimage at `image.output` and reads what that execution prints, using the commands `tests/README.md` lists.

A test therefore says what a user sees, and a change to how flatimage stores a setting leaves the test untouched.

## A case fills the types, never YAML text

A case builds the FlatCompose file as the product's own types, and `Harness::write` turns them into the file. No case assembles YAML text by hand.

A case writes raw text only when it states a key or a value the types cannot hold.

## A tag names the rule

Every test function carries a `// covers: <id>` tag naming the spec item it checks, and `tests/README.md` holds the row that matches it. The tags and the rows hold the same set, so a read of these files together finds any gap.

A rule no spec page states gets no test. A case that checks a behaviour the spec does not state gets the spec page changed first.

## Each test target declares the harness

Cargo makes one test target of each `tests/<block>/main.rs`. The directory `tests/harness/` holds no `main.rs`, so it stays out of that discovery and the harness has one copy. Each target declares the harness files its cases reach, by path:

```rust
#[path = "../harness/common.rs"]
mod common;

#[path = "../harness/flatimage.rs"]
mod flatimage;

#[path = "../harness/mock/mod.rs"]
mod mock;
```

`harness/flatimage.rs` wraps flatimage's own command model, cut to the command families the spec reads. A family a case needs is copied whole, so a reader can compare it with flatimage's own CLI.

## A value is compared in a form flatimage keeps

flatimage stores a key as a set or reports its entries sorted by name. A case compares a list in the form flatimage keeps, and never asserts the order the FlatCompose file wrote.

A refusal is read as `outcome.refused_with(needle)`. Where a refusal happens before the digest step, the case also checks that no digest file stands beside the output.

## The spec writes the tests

A case states a rule a spec page states, and a case is written from that page. A case can exist before the code it checks, and only the spec defines what a case checks.

When a case fails, the code is wrong. A case is never weakened, deleted or rewritten to match the code it caught.

A case changes only when the spec changes, in the same change as the spec.

## A test never turns itself off

No `#[ignore]`, no `cfg` that removes a case, and no early return that skips a check. A case that cannot run fails with a message naming what is missing, which is how a missing `FLATIMAGE` behaves.
