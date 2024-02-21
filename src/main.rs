use clap::{command, Arg, ArgAction, Command};
use env_logger;
use log;
use std::env;
use std::error;
use std::os::unix::process::CommandExt;
use std::process;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    let cwd = env::current_dir()?;
    let cli = mk::CommandLineParser::new();

    // Traverse upwards through the filesystem to find a Makefile.
    let mut makefile = mk::Makefile::new();
    let _ = makefile.find_and_parse();
    log::debug!("Found Makefile at: {:?}", makefile.cwd);

    if !cli.has_flags {
        log::debug!("Will execute and exec: make {}", cli.join(" "));

        env::set_current_dir(makefile.cwd.as_path())?;
        process::Command::new("make").args(&cli.args).exec();
        // We shouldn't reach this point. But if we do, let's try to
        // clean up and move back to the original working directory.
        env::set_current_dir(cwd.as_path())?;
        process::exit(1)
    }

    // Create a new parser for command line arguments.
    // This is used solely to generate help text.
    let mut cmd: Command = command!()
        .arg(
            Arg::new("me")
                .short('m')
                .long("me")
                .help("Run an internal `mk` command")
                .action(ArgAction::SetTrue),
        )
        .disable_help_subcommand(true);

    // Add additional help text and a subcommand for each target found in the Makefile.
    cmd = cmd.after_help(makefile.postscript);
    for (target, help) in makefile.targets {
        cmd = cmd.subcommand(Command::new(target).about(help));
    }

    // Store the rendered help text for later use.
    let help_text = cmd.render_help();

    // This finalizes the argument parser and takes ownership of `cmd`.
    let matches = cmd.get_matches();

    // --me, or -m, was passed to indicate an internal command.
    // None exist, so we don't do anything else.
    if matches.get_flag("me") {
        log::debug!("No internal commands exist.");
        return Ok(());
    }

    eprintln!("{}", help_text);
    Ok(())
}
