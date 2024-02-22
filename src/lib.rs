use clap::{command, Arg, ArgAction, Command};
use log;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::error;
use std::fs;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process;

pub struct CommandLine {
    pub args: Vec<String>,
    pub flags_found: bool,
}

impl CommandLine {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let flags_found = args.iter().any(|a| a.starts_with('-'));

        CommandLine { args, flags_found }
    }

    pub fn run(
        &self,
        program: String,
        args: &Vec<String>,
        cwd: &Path,
    ) -> Result<(), Box<dyn error::Error>> {
        let pwd = std::env::current_dir()?;
        log::debug!("Will execute and exec: make {}", args.join(" "));

        env::set_current_dir(cwd)?;
        process::Command::new(program).args(args).exec();

        // We shouldn't reach this point. But if we do, let's try to
        // clean up and move back to the original working directory.
        env::set_current_dir(pwd)?;
        process::exit(1);
    }
}

impl Default for CommandLine {
    fn default() -> Self {
        CommandLine::new()
    }
}

pub struct Makefile {
    pub targets: Vec<(String, String)>,
    pub cwd: PathBuf,
    pub postscript: String,
    postscript_locked: bool,
}

impl Default for Makefile {
    fn default() -> Self {
        Makefile::new()
    }
}

impl Makefile {
    pub fn new() -> Self {
        Makefile {
            targets: Vec::new(),
            cwd: PathBuf::new(),
            postscript: String::new(),
            postscript_locked: false,
        }
    }

    pub fn find_and_parse(&mut self) -> io::Result<()> {
        let location = self.find()?;
        let lines = self.read(location.as_path())?;
        self.parse(lines);
        Ok(())
    }

    fn find(&mut self) -> io::Result<PathBuf> {
        self.cwd = std::env::current_dir()?;
        loop {
            log::debug!("Searching for Makefile in: {:?}", self.cwd);
            let makefile_path = self.cwd.join("Makefile");
            if makefile_path.exists() {
                return Ok(makefile_path);
            }

            let dot_git_path = self.cwd.join(".git");
            if dot_git_path.exists() {
                log::debug!("Reached the root of the project.");
            }

            if !self.cwd.pop() || dot_git_path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No Makefile found in the project!",
                ));
            }
        }
    }

    fn read(&self, path: &Path) -> io::Result<Vec<String>> {
        let content = fs::read_to_string(path)?;
        let mut lines: Vec<String> = Vec::new();

        for line in content.lines() {
            lines.push(line.to_string());
        }

        Ok(lines)
    }

    fn parse(&mut self, lines: Vec<String>) {
        let mut seen: HashSet<String> = HashSet::new();
        let mut comments = Vec::new();

        let target_re = Regex::new(r"^([a-zA-Z0-9%_-]+):.*").unwrap();
        let comment_re = Regex::new(r"^#>\s(.*)").unwrap();

        for line in lines {
            // If the line is a comment, add it to the comments vec.
            if let Some(captures) = comment_re.captures(&line) {
                comments.push(captures[1].to_string());

            // If the line is a make target, add it to the targets vec.
            } else if let Some(captures) = target_re.captures(&line) {
                let target = captures[1].to_string();
                let help = comments.join("\n");

                log::debug!("Found make target: {}", target);

                if !seen.contains(&target) {
                    log::debug!("Adding target: {}", target);
                    seen.insert(target.clone());
                    self.targets.push((target, help));
                }
                self.postscript_locked = true;

                comments.clear();

            // Otherwise, check if we've seen any make targets, and if not,
            // populate the postscript message. If we've passed the first make
            // target, then drop any future dangling comments.
            } else {
                if !self.postscript_locked {
                    for comment in &comments {
                        self.postscript.push_str(comment.as_str());
                    }
                }

                comments.clear();
            }
        }
    }

    pub fn get_arg_parser(&mut self) -> Command {
        // Create a new parser for command line arguments.
        // This is used solely to generate help text.
        let mut cmd: Command = command!()
            .arg(
                Arg::new("me")
                    .long("me")
                    .help("Run an internal `mk` command")
                    .action(ArgAction::SetTrue),
            )
            .disable_help_subcommand(true);

        // Add additional help text and a subcommand for each target found in the Makefile.
        cmd = cmd.after_help(&self.postscript);
        for (target, help) in &self.targets {
            cmd = cmd.subcommand(Command::new(target).about(help));
        }

        cmd
    }
}
