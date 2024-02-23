use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;

use crate::common::Space;
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
            "DEBUG mk] Reached the root of the project.",
        ));

    Ok(())
}

#[test]
fn prints_minimal_help_if_makefile_is_empty() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("blank")?;

    space
        .bin()
        .assert()
        .success()
        .stderr(predicate::str::contains("Usage: mk [OPTIONS]"));

    Ok(())
}

#[test]
fn can_execute_make_with_valid_syntax_repeated_targets() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("repeats")?;

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
        .arg("-B")
        .assert()
        .success()
        .stdout(predicate::str::contains("This is a successful test."));

    Ok(())
}

#[test]
fn uses_env_vars_preceding_the_command() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("envvars")?;

    space
        .bin()
        .env("ENVVAR", "success") // This may not be sufficient. Call from bash?
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("ENVVAR is success."));

    Ok(())
}

#[test]
fn uses_env_vars_passed_as_arguments() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("envvars")?;

    space
        .bin()
        .arg("test")
        .arg("ENVVAR=success")
        .assert()
        .success()
        .stdout(predicate::str::contains("ENVVAR is success."));

    Ok(())
}

#[test]
fn shows_make_error_if_an_invalid_target_is_used() -> Result<(), Box<dyn Error>> {
    let space = Space::from_fixture("test")?;

    space
        .bin()
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::is_match(
            r###"\*\*\* No rule to make target [`']invalid'"###,
        ).unwrap());  // Some versions of make use a leading backtick, some don't.

    Ok(())
}
