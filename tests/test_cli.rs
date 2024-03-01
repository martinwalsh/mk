use assert_cmd::prelude::*;
use insta_cmd::assert_cmd_snapshot;
use predicates::prelude::*;
use std::error::Error;

use crate::common::{dedent, Space};
mod common;

#[test]
fn returns_an_error_if_the_makefile_is_missing() -> Result<(), Box<dyn Error>> {
    let space = Space::new();

    space
        .bin()
        .assert()
        .failure()
        .stderr(predicate::str::contains("No Makefile found"));

    Ok(())
}

#[test]
fn traversal_stops_at_the_dot_git_directory() -> Result<(), Box<dyn Error>> {
    let space = Space::new();

    space
        .bin()
        .env("RUST_LOG", "DEBUG")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "ERROR mk] No Makefile found in the project!",
        ))
        .stderr(predicate::str::contains(
            "DEBUG mk::makefile] Reached the root of the project.",
        ));

    Ok(())
}

#[test]
fn prints_minimal_help_if_makefile_is_empty_and_no_args_given() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("blank")?;
    assert_cmd_snapshot!(space.bin());
    Ok(())
}

#[test]
fn prints_minimal_help_if_makefile_is_empty_and_h_flag_is_given() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("blank")?;
    assert_cmd_snapshot!(space.bin().arg("-h"));
    Ok(())
}

#[test]
fn prints_minimal_help_if_makefile_is_empty_and_help_flag_is_given() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("blank")?;
    assert_cmd_snapshot!(space.bin().arg("--help"));
    Ok(())
}

#[test]
fn can_execute_make_with_valid_syntax_repeated_targets() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("repeats")?;

    assert_cmd_snapshot!(space.bin().arg("test"));

    space
        .bin()
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("This is a test var: success"));

    Ok(())
}

#[test]
fn can_take_make_flags_like_force_build() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("test")?;

    space
        .bin()
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("This is a successful test."));

    space
        .bin()
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains(" is up to date."));

    space
        .bin()
        .arg("test")
        .arg("-B")
        .assert()
        .success()
        .stdout(predicate::str::contains("This is a successful test."));

    Ok(())
}

#[test]
fn uses_env_vars_preceding_the_command() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("envvars")?;

    assert_cmd_snapshot!(space.bin().env("ENVVAR", "success").arg("test"));

    Ok(())
}

#[test]
fn uses_env_vars_passed_as_arguments() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("envvars")?;

    assert_cmd_snapshot!(space.bin().arg("test").arg("ENVVAR=success"));

    Ok(())
}

#[test]
fn shows_make_error_if_an_invalid_target_is_used() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("test")?;

    space.bin().arg("invalid").assert().failure().stderr(
        predicate::str::is_match(r###"\*\*\* No rule to make target [`']invalid'"###).unwrap(),
    ); // Some versions of make use a leading backtick, some don't.

    Ok(())
}

#[test]
fn shows_help_with_documentation() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("docs")?;
    assert_cmd_snapshot!(space.bin().arg("--help"));

    space
        .bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("test  Runs the tests"));

    Ok(())
}

#[test]
fn skips_help_for_targets_without_documentation() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("nodocs")?;

    assert_cmd_snapshot!(space.bin().arg("--help"));

    space
        .bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-me").count(0));

    Ok(())
}

#[test]
fn can_parse_help_from_includes() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("include")?;

    let content = dedent(
        r###"
        #| Included test target
        included:
        \techo 'This is another successful test.'
        .PHONY: included
    "###,
    )
    .replace(r"\t", "\t");
    space.add_include("includes", "include", &content)?;

    assert_cmd_snapshot!(space.bin().arg("--help"));

    Ok(())
}

#[test]
fn can_run_included_commands_from_a_subfolder() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("include")?;

    let content = dedent(
        r###"
        #| Included test target
        included:
        \t@echo 'This is another successful test.'
        .PHONY: included
    "###,
    )
    .replace(r"\t", "\t");
    space.add_include("includes", "include", &content)?;

    assert_cmd_snapshot!(space.bin().arg("included"));

    Ok(())
}
