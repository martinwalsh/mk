use regex::Regex;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct CommandLineParser {
    pub args: Vec<String>,
    pub debug_mode: bool,
    pub has_flags: bool,
}

impl CommandLineParser {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let has_flags = args.iter().any(|a| a.starts_with('-'));

        CommandLineParser {
            args: args,
            has_flags: has_flags,
            debug_mode: std::env::var("MK_DEBUG").is_ok(),
        }
    }

    pub fn join(&self, sep: &str) -> String {
        self.args.join(sep)
    }
}

impl Default for CommandLineParser {
    fn default() -> Self {
        CommandLineParser::new()
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
            let location = self.cwd.join("Makefile");
            if location.exists() {
                return Ok(location);
            }

            if !self.cwd.pop() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No Makefile found!",
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

                self.targets.push((target, help));
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
}
