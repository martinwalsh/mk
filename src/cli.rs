use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::{env, error, process};

pub fn makecmd(args: &Vec<OsString>, cwd: &Path) -> Result<(), Box<dyn error::Error>> {
    log::debug!("Will execute and exec: make | {:?}", args);

    // Keep track of the calling cwd for use in the Makefile.
    env::set_var("MK_CWD", env::current_dir()?.as_os_str());

    process::Command::new("make")
        .args(args)
        .current_dir(cwd)
        .exec();

    // We shouldn't reach this point.
    // But if we do, let's try to clean up.
    process::exit(1);
}

pub struct RawArgumentParser {
    pub raw_args: Vec<OsString>,
    pub has_raw_args: bool,
}

impl RawArgumentParser {
    pub fn new() -> Self {
        let raw_args: Vec<OsString> = std::env::args_os().skip(1).collect::<Vec<OsString>>();
        let has_raw_args = raw_args.iter().any(|a| match a.to_str() {
            Some(s) => !s.starts_with('-'),
            None => false,
        });

        RawArgumentParser {
            raw_args,
            has_raw_args,
        }
    }
}
