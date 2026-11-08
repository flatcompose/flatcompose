# The FlatCompose test suite

Every `// covers:` tag in `tests/` has one row in the tables below, and every row states the tagged function, so the file and the tree agree.

## The tiers

A unit test stands beside the source file that lowers a key, and it checks the words and the order of one command's arguments, which the file at `image.output` cannot report. No integration test repeats that work.

An integration test stands under `tests/` and runs the binary `cargo` builds for the package. It reads the result back through flatimage's own verbs. It checks the published file and the settings flatimage reports from that file.

## Compose

`compose.md` states the `image` block. The cases live in `tests/image/image.rs`.

| Rule | The claim one case checks | The case |
|---|---|---|
| `FR-IMAGE-001` | A file stating `from` and `output` alone writes the file at `image.output` and its digest, and the run doesn't print anything. | `image.rs::a_file_carrying_only_the_image_block_builds` |
| `FR-IMAGE-001` | A run given no `-f` reads `flatcompose.yml` from its working directory and writes the file `output` states. | `image.rs::the_default_file_name_serves_a_build_without_the_flag` |
| `FR-IMAGE-001` | A file that omits `output`, and a file that omits `image`, each get refused with the field named. | `image.rs::a_key_the_schema_wants_stays_required` |
| `FR-IMAGE-001` | A file whose YAML does not parse exits non-zero and prints nothing on stdout. | `image.rs::a_file_that_does_not_parse_is_refused` |
| `FR-IMAGE-001` | `-f` naming an absent path exits non-zero and prints that path on stderr. | `image.rs::a_file_that_is_not_there_is_refused` |
| `FR-IMAGE-001` | An `arch` list naming a sixth token, and a sequence where `image` wants a mapping, each get refused. | `image.rs::a_bad_value_inside_the_image_block_is_refused` |
| `FR-IMAGE-002` | The file `from` names keeps the digest the build found it with. | `image.rs::the_file_from_names_stays_as_the_build_found_it` |
| `FR-IMAGE-002` | A relative `from` resolves against the directory holding the FlatCompose file, not the working directory. | `image.rs::a_from_path_resolves_beside_the_flatcompose_file` |
| `FR-IMAGE-002` | A second run that omits `from` starts from the file at `image.output`. | `image.rs::without_from_the_build_starts_from_the_file_at_output` |
| `FR-IMAGE-002` | An `http` URL in `from` builds the file at `image.output`. | `image.rs::a_url_from_gets_fetched_once` |
| `FR-IMAGE-002` | A local `from` that does not exist exits non-zero before the digest step. | `image.rs::a_from_that_is_not_there_is_refused` |
| `FR-IMAGE-003` | `sha256sum -c app.flatimage.sha256sum` run from the test directory exits zero. | `image.rs::the_output_file_verifies_with_sha256sum` |
| `FR-IMAGE-003` | The digest file states the digest, the separator `sha256sum -c` expects, and the file name. | `image.rs::the_digest_line_names_the_digest_and_the_file` |
| `FR-IMAGE-003` | The printed digits equal the SHA-256 the harness computes over the file at `image.output`. | `image.rs::the_digest_matches_the_output_bytes` |
| `FR-IMAGE-004` | `inplace: false` replaces the permissions an earlier run wrote with the settings of the later run. | `image.rs::inplace_false_replaces_what_the_output_held` |
| `FR-IMAGE-004` | `inplace: true` keeps what the file at `image.output` held and adds the settings of the later run. | `image.rs::inplace_true_keeps_what_the_output_held` |
| `FR-IMAGE-004` | `inplace: true` with no file at `image.output` writes one built from `from`. | `image.rs::inplace_true_creates_the_output_from_from` |
| `US1-2` | A second run of one unchanged file leaves a file reporting the same settings. | `image.rs::a_second_build_of_one_file_reports_the_same_settings` |
| `US1-3` | A key beside `image` exits non-zero and prints that key on stderr, and the run doesn't leave a file at `image.output`. | `image.rs::an_unknown_top_level_key_is_refused_naming_it` |
| `US1-3` | An unknown key inside `image`, and an unknown key inside `config`, each get refused with the key on stderr. | `image.rs::an_unknown_key_inside_a_block_is_refused_naming_it` |
| `FR-IMAGE-005` | A file listing every architecture token builds and writes the file at `image.output`. | `image.rs::every_arch_token_is_accepted` |
| `FR-IMAGE-005` | An `arch` list naming `mips64` exits non-zero and prints `mips64` on stderr, with nothing on stdout. | `image.rs::an_arch_token_outside_the_valid_values_is_refused` |
| `FR-IMAGE-005` | A file that omits `arch` builds. | `image.rs::an_absent_arch_list_builds` |
| `FR-IMAGE-006` | A file stating `output` alone, with no file there, gets refused with the message `compose.md` fixes. | `image.rs::a_build_with_nothing_to_start_from_is_refused` |

## Configure

`configure.md` states one section per rule family, and the table below follows the page's order. The cases live in `tests/config/`, one file for each family with a case.

| Rule | The claim one case checks | The case |
|---|---|---|
| `FR-CONFIG-001` | A file stating `image` alone builds, and its reading verbs don't print anything. | `config.rs::a_file_stating_no_config_block_builds` |
| `FR-CONFIG-001` | A file writing `config: {artifacts: []}` exits non-zero and prints `artifacts` on stderr. | `config.rs::an_unknown_key_inside_the_config_block_is_refused` |
| `FR-CONFIG-002` | One file whose `config` block states settings leaves a file reporting each of those settings. | `config.rs::every_key_of_the_config_block_reaches_the_output_file` |
| `FR-CONFIG-003` | An environment value of `a=b c` reads back as `a=b c`. | `config.rs::a_value_carrying_spaces_and_equals_arrives_unchanged` |
| `FR-CONFIG-003` | `perms: [nonsense]` exits non-zero and prints flatimage's refusal on stderr. The run doesn't leave a digest file. | `config.rs::a_value_the_flatimage_refuses_ends_the_build` |
| `FR-CONFIG-002` | A failing step leaves the settings of the config block unapplied. | `steps/steps.rs::the_config_block_runs_only_after_the_steps_finish` |
| `FR-PERMS-001` | The permissions the case wrote read back as those names. | `perms.rs::perms_reach_the_output_file` |
| `FR-PERMS-001` | A shorter list reads back the same way. | `perms.rs::a_single_permission_reaches_the_output_file` |
| `FR-CAPSULE-001` | The capsules the case wrote read back as those names. | `capsule.rs::capsule_reaches_the_output_file` |
| `FR-CAPSULE-001` | A shorter list reads back the same way. | `capsule.rs::a_single_capsule_reaches_the_output_file` |
| `FR-UNSHARE-001` | The namespaces the case wrote read back as those names. | `unshare.rs::unshare_reaches_the_output_file` |
| `FR-UNSHARE-001` | A shorter list reads back the same way. | `unshare.rs::a_single_namespace_reaches_the_output_file` |
| `FR-OPTIONS-001` | `casefold` reads back as that name. | `options.rs::options_reach_the_output_file` |
| `FR-OPTIONS-001` | A longer list reads back as the names it wrote. | `options.rs::two_options_reach_the_output_file` |
| `FR-OVERLAY-001` | `overlayfs` reads back from `fim-overlay show`. | `overlay.rs::overlay_reaches_the_output_file` |
| `FR-OVERLAY-001` | An unknown backend exits non-zero before the digest step. | `overlay.rs::a_backend_the_flatimage_does_not_know_ends_the_build` |
| `FR-SANDBOX-001` | `bwrap` reads back from `fim-sandbox show`. | `sandbox.rs::sandbox_reaches_the_output_file` |
| `FR-SANDBOX-001` | `none` reads back from `fim-sandbox show` as the mode the case wrote. | `sandbox.rs::the_other_mode_reaches_the_output_file` |
| `FR-ENV-001` | `LANG: C.UTF-8` reads back with that name and value. | `env.rs::env_reaches_the_output_file` |
| `FR-ENV-001` | A mapping with more entries reads back as the pairs it wrote. | `env.rs::two_variables_reach_the_output_file` |
| `FR-ENV-001` | `ARGS: a=b` reads back with the value `a=b`. | `env.rs::a_value_carrying_an_equals_sign_keeps_its_tail` |
| `FR-BOOT-001` | A program with arguments reads back with its arguments. | `boot.rs::boot_reaches_the_output_file` |
| `FR-BOOT-001` | A program stated alone reads back without arguments. | `boot.rs::a_program_without_args_reaches_the_output_file` |
| `FR-BIND-001` | One binding reads back with its `src`, `dst` and `mode`. | `bind.rs::bind_reaches_the_output_file` |
| `FR-BIND-001` | A list of bindings reads back with each binding's own mode. | `bind.rs::two_bindings_reach_the_output_file` |
| `FR-BIND-001` | `mode: rwx` exits non-zero and prints `rwx` with the `bind` entry it belongs to on stderr. | `bind.rs::a_mode_outside_the_valid_values_is_refused` |
| `FR-BIND-001` | A binding that omits `mode`, and a binding that omits `dst`, each get refused with the field named. | `bind.rs::a_binding_missing_a_field_is_refused` |
| `FR-DESKTOP-001` | The manifest reads back with the name and the categories the case wrote. | `desktop.rs::a_manifest_reaches_the_output_file` |
| `FR-DESKTOP-001` | The icon reads back with the bytes of the PNG the case wrote. | `desktop.rs::an_icon_reaches_the_output_file` |
| `FR-DESKTOP-001` | The entry reads back with the `.desktop` text the case wrote. | `desktop.rs::an_entry_reaches_the_output_file` |
| `FR-DESKTOP-001` | The mimetype reads back with the XML text the case wrote. | `desktop.rs::a_mimetype_reaches_the_output_file` |
| `FR-DESKTOP-001` | Integration items stored beside the manifest read back as the items the case wrote. | `desktop.rs::integrations_reach_the_output_file` |
| `FR-DESKTOP-001` | Every desktop field stated together answers on its own noun, and the run doesn't print anything. | `desktop.rs::every_desktop_field_reaches_the_output_file` |
| `FR-DESKTOP-001` | An icon path that does not exist exits non-zero before the digest step. | `desktop.rs::a_desktop_path_that_is_not_there_ends_the_build` |

## Steps

`steps.md` states the `steps` block and the `builder` and `exec` commands. The cases live in `tests/steps/`, one file per family: `steps.rs` for `FR-STEPS`, `builder.rs` for `FR-BUILDER`, and `exec.rs` for `FR-EXEC`.

| Rule | The claim one case checks | The case |
|---|---|---|
| `FR-STEPS-001` | A step stating `system` instead of `builder` exits non-zero naming `builder`, and doesn't leave a digest file. | `steps/steps.rs::a_step_stating_a_command_other_than_builder_is_refused` |
| `FR-STEPS-002` | A file whose steps block states one builder step builds silently, and `fim-root system list` names the root the source names. | `steps/steps.rs::a_builder_step_builds_the_root_it_names` |
| `FR-STEPS-002` | A step whose source the install cannot use runs a `fim-root` command, and the build ends when that command fails. | `steps/steps.rs::a_builder_step_runs_a_command_against_the_flatimage` |
| `FR-BUILDER-002` | A step that omits `from` exits non-zero naming the field, and doesn't leave a digest file. | `steps/builder.rs::a_step_that_names_no_source_is_refused` |
| `FR-BUILDER-003` | A step that omits the install request exits non-zero naming the field, and doesn't leave a digest file. | `steps/builder.rs::a_step_that_names_no_install_request_is_refused` |
| `FR-BUILDER-006` | A builder with an empty patterns list exits non-zero naming `steps[0].builder.install.patterns`, and doesn't leave a digest file. | `steps/builder.rs::a_builder_whose_install_names_no_pattern_is_refused` |
| `FR-STEPS-003` | An `exec` step placed before the `builder` that creates its root exits non-zero naming the missing root, and doesn't leave a digest file. | `steps/exec.rs::an_exec_step_before_the_step_that_builds_its_root_is_refused` |
| `FR-STEPS-003` | Two `exec` steps of one build run in the order the file writes them, the second reading a file the first wrote. | `steps/exec.rs::an_exec_step_runs_every_script_in_the_order_the_file_writes_them` |
| `FR-STEPS-004` | A file whose steps block holds an `exec` step builds. | `steps/exec.rs::an_exec_step_runs_its_script_in_the_root_it_names` |
| `FR-EXEC-001` | A step stating `exec` runs `fim-exec` against the FlatImage. | `steps/exec.rs::an_exec_step_runs_its_script_in_the_root_it_names` |
| `FR-EXEC-002` | A step that omits `from` exits non-zero naming the field, and doesn't leave a digest file. | `steps/exec.rs::an_exec_step_that_names_no_source_is_refused` |
| `FR-EXEC-003` | An `exec` step runs in the root its `from` names, not the root the earlier steps left active. | `steps/exec.rs::an_exec_step_runs_its_script_in_the_root_it_names` |
| `FR-EXEC-003` | An `exec` step run before the step that builds its root exits non-zero naming the missing root, and doesn't leave a digest file. | `steps/exec.rs::an_exec_step_before_the_step_that_builds_its_root_is_refused` |
| `FR-EXEC-003` | The lowered command selects the root before running `fim-exec`. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_selects_the_root_before_running_the_script` |
| `FR-EXEC-003` | FlatCompose tells `fim-root` to use the name `from` derives. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_names_the_root_the_source_derives` |
| `FR-EXEC-004` | The lowered command runs `bash` on the script path. The script text never enters the argument list. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_runs_bash_with_the_script_path` |
| `FR-EXEC-004` | The build writes the script to `/tmp/fim/flatcompose/<build-pid>/script-<n>.sh`, `<n>` counting steps from zero. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_names_the_script_by_the_step_position` |
| `FR-EXEC-004` | A script with quotes, substitutions and a heredoc arrives verbatim. | `steps/exec.rs::an_exec_step_runs_its_script_text_verbatim` |
| `FR-EXEC-004` | The step's position sets the file name that bash runs: a failing second step prints `script-1.sh` on stderr. | `steps/exec.rs::an_exec_step_names_its_script_by_the_step_position` |
| `FR-EXEC-004` | A step that succeeds leaves its script file behind, under the build's pid directory. | `steps/exec.rs::an_exec_step_runs_every_script_in_the_order_the_file_writes_them` |
| `FR-EXEC-004` | A step that omits `script` exits non-zero naming the field, and doesn't leave a digest file. | `steps/exec.rs::an_exec_step_that_names_no_script_is_refused` |
| `FR-EXEC-004` | A step with an empty `script` exits non-zero naming `steps[0].exec.script`, and doesn't leave a digest file. | `steps/exec.rs::an_exec_step_whose_script_is_empty_is_refused` |
| `FR-EXEC-005` | A step naming `uid: 1000` runs its script with `$EUID` 1000. | `steps/exec.rs::an_exec_step_runs_its_script_as_the_uid_and_gid_it_names` |
| `FR-EXEC-005` | An absent `uid` omits the flag. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_omits_the_flags_the_file_does_not_name` |
| `FR-EXEC-005` | An explicit `uid: 0` stays in the command. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_keeps_an_explicit_zero_uid_and_gid` |
| `FR-EXEC-006` | A step naming `gid: 4242` runs its script with `$GROUPS[0]` 4242. | `steps/exec.rs::an_exec_step_runs_its_script_as_the_uid_and_gid_it_names` |
| `FR-EXEC-006` | An absent `gid` omits the flag. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_omits_the_flags_the_file_does_not_name` |
| `FR-EXEC-006` | The command writes `--uid` before `--gid`. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_writes_the_uid_before_the_gid` |
| `FR-EXEC-006` | An explicit `gid: 0` stays in the command. | [../src/compose/steps/exec.rs](../src/compose/steps/exec.rs)::`lower_keeps_an_explicit_zero_uid_and_gid` |

## Reading a value back

`FR-CONFIG-002` says a build runs the command of every key. A case turns that claim into an observation by reading the setting with flatimage's own verb.

| Key | The verb that reports it |
|---|---|
| `perms` | `fim-perms list` |
| `options` | `fim-options list` |
| `unshare` | `fim-unshare list` |
| `capsule` | `fim-capsule list` |
| `overlay` | `fim-overlay show` |
| `sandbox` | `fim-sandbox show` |
| `env` | `fim-env list`, one name and value per entry |
| `boot` | `fim-boot show`, the program and its arguments |
| `bind` | `fim-bind list`, one `src`, `dst` and `mode` per binding |
| `desktop` | `fim-desktop manifest show`, `icon show`, `entry show`, `mimetype show` and `integrations show` |

flatimage stores a key as a set, or reports its entries sorted by name. A case compares a list in the form flatimage keeps and never relies on the order the FlatCompose file wrote.

## Running the suite

The suite runs in the container that `docker/Dockerfile.tests` builds, and never on a host. `tests/AGENTS.md` states that rule and the decisions the suite rests on.
