use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // let cwd = env::current_dir()?;
    let cli = mk::CommandLine::new();

    let mut makefile = mk::Makefile::new();
    log::debug!("Found Makefile at: {:?}", makefile.cwd);

    // Traverse upwards through the filesystem, stop if we find a
    // Makefile and parse it, or if we see a .git directory signifying
    // the root of the project.
    match makefile.find_and_parse() {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e);
            process::exit(1);
        }
    }

    if cli.args_found {
        cli.run("make".to_string(), &cli.args, &makefile.cwd)?;
    }

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
