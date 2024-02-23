use assert_cmd::prelude::*;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

pub struct Space {
    pub tempdir: TempDir,
    pub workdir: PathBuf,
}

impl Space {
    pub fn new() -> Self {
        let tempdir = TempDir::with_prefix(".mk-tests---").unwrap();
        let workdir = tempdir.path().join("workspace");

        let gitdir = workdir.join(".git");
        fs::create_dir_all(&gitdir).ok();

        Self {
            tempdir: tempdir,
            workdir: workdir,
        }
    }

    pub fn bin(&self) -> Command {
        let mut cmd = Command::cargo_bin("mk").unwrap();
        cmd.current_dir(&self.workdir);
        cmd
    }

    fn fixture(name: String) -> String {
        let this = file!();
        let thisdir = Path::new(this).parent().unwrap();
        let fixture = thisdir.join("fixtures").join(name).with_extension("mk");

        fs::read_to_string(fixture).unwrap()
    }

    pub fn from_fixture(name: &str) -> Result<Space, Box<dyn Error>> {
        let space = Self::new();
        let content = Space::fixture(name.to_string());
        let makefile = space.workdir.join("Makefile");

        fs::write(&makefile, content)?;

        Ok(space)
    }
}
