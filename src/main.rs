use std::error::Error;
use std::process;

use crate::makefile::Makefile;

mod cli;
mod help;
mod makefile;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Traverses upwards in the filesystem, stops if we find a Makefile
    // and then parses it (or stops and fails if a .git directory is found first).
    let makefile = match Makefile::new(None) {
        Ok(m) => m,
        Err(e) => {
            log::error!("{}", e);
            process::exit(1);
        }
    };
    log::debug!("Using Makefile at: {:?}", makefile.path);

    let cli = cli::RawArgumentParser::new();
    if cli.has_raw_args {
        if let Some(parent) = makefile.path.parent() {
            cli::makecmd(&cli.raw_args, parent)?;
        }
    }

    let mut parser = help::make_help(makefile.about, makefile.epilogue, makefile.targets);

    // Store the rendered help text for later use.
    let help = parser.render_help();

    // This finalizes the argument parser and takes ownership.
    let matches = parser.get_matches();

    // The --me flag was passed to indicate an internal command.
    // None exist right now, so we don't do anything else.
    if matches.get_flag("me") {
        log::debug!("No internal --me commands exist.");
        return Ok(());
    }

    eprintln!("{}", help);
    Ok(())
}
