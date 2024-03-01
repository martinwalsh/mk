use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;

pub struct Makefile {
    pub path: PathBuf,
    pub about: String,
    pub postscript: String,
    pub targets: Vec<(String, String)>,
}

impl Makefile {
    pub fn new(makefile: Option<PathBuf>) -> Result<Self, io::Error> {
        let path = match makefile {
            Some(p) => p,
            None => Self::find(env::current_dir()?)?,
        };
        let parsed = Self::parse(&path, true)?;
        Ok(parsed)
    }

    fn find(mut cwd: PathBuf) -> io::Result<PathBuf> {
        loop {
            log::debug!("Searching for Makefile in: {:?}", cwd);
            let makefile_path = cwd.join("Makefile");
            if makefile_path.exists() {
                return Ok(makefile_path);
            }

            let dot_git_path = cwd.join(".git");
            if dot_git_path.exists() {
                log::debug!("Reached the root of the project.");
            }

            if !cwd.pop() || dot_git_path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No Makefile found in the project!",
                ));
            }
        }
    }

    //fn parse(path: &Path, check_includes: bool) -> io::Result<(String, String, Vec<(String, String)>)> {
    fn parse(path: &Path, check_includes: bool) -> io::Result<Makefile> {
        log::debug!("Parsing Makefile: {:?}", path);

        let content = fs::read_to_string(path)?;

        let mut about = Vec::new();
        let mut postscript = Vec::new();

        let mut seen = HashSet::new();
        let mut targets = Vec::new();
        let mut comments = Vec::new();
        let mut includes = Vec::new();

        let target_re = Regex::new(r"^([a-zA-Z0-9/%_-]+):.*").unwrap();
        let include_re = Regex::new(r"^-?include\s(.*)").unwrap();

        for line in content.lines() {
            // If the line is a `#|` prefixed comment, add it to the comments vec.
            if let Some(s) = line.strip_prefix("#|") {
                comments.push(s.trim().to_string());

            // If the line is a `#>` prefixed comment, add it to the postscript vec.
            } else if let Some(s) = line.strip_prefix("#>") {
                postscript.push(s.trim().to_string());

            // If the line is a `#<` prefixed comment, add it to the about vec.
            } else if let Some(s) = line.strip_prefix("#<") {
                about.push(s.trim().to_string());

            // If the line is a make target, add it to the targets vec.
            } else if let Some(caps) = target_re.captures(line) {
                let target = caps[1].trim().to_string();
                let help = comments.join("\n").trim().to_string();
                let has_comments = !comments.is_empty();
                log::debug!("Found target: {}", target);

                if !seen.contains(&target) {
                    // If we don't have help for this target,
                    // don't include the target at all.
                    if has_comments {
                        log::debug!("Adding target: {}", target);
                        seen.insert(target.clone());
                        targets.push((target, help));
                    }
                }

                comments.clear();
            } else if check_includes {
                // If the line is an include statement, add it to the includes vec.
                if let Some(caps) = include_re.captures(line) {
                    let include_path = PathBuf::from(caps[1].trim().to_string());
                    log::debug!("Found include: {:?}", include_path);

                    includes.push(include_path);
                }
            }
        }

        for include_path in includes {
            if !include_path.exists() {
                log::debug!("Include file not found: {:?}", include_path);
                continue;
            }
            // let (_, _, include_targets) = Makefile::parse(&include_path, false)?;
            let include = Makefile::parse(&include_path, false)?;
            for (target, help) in include.targets {
                if !seen.contains(&target) {
                    seen.insert(target.clone());
                    targets.push((target, help));
                }
            }
        }

        Ok(Makefile {
            targets,
            path: path.to_path_buf(),
            about: about.join("\n"),
            postscript: postscript.join("\n"),
        })
    }
}
