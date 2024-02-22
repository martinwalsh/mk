use env_logger;
use log;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // let cwd = env::current_dir()?;
    let cli = mk::CommandLine::new();

    // Traverse upwards through the filesystem to find a Makefile.
    let mut makefile = mk::Makefile::new();
    match makefile.find_and_parse() {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e);
            process::exit(1);
        }
    }
    log::debug!("Found Makefile at: {:?}", makefile.cwd);

    if !cli.flags_found {
        cli.run("make".to_string(), &cli.args, &makefile.cwd)?;
    }

    // let mut cmd = generate_help(makefile);

    let mut parser = makefile.get_arg_parser();

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
