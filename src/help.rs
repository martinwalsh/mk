use clap::{command, Arg, ArgAction, Command};

pub fn make_help(about: String, postscript: String, targets: Vec<(String, String)>) -> Command {
    let mut cmd = command!()
        .arg(
            Arg::new("me")
                .long("me")
                .help("Run an internal `mk` command")
                .action(ArgAction::SetTrue),
        )
        .disable_help_subcommand(true);

    if !about.is_empty() {
        cmd = cmd.about(about);
    }

    if !postscript.is_empty() {
        cmd = cmd.after_help(postscript);
    }

    for (target, help) in targets {
        cmd = cmd.subcommand(Command::new(target).about(help));
    }

    cmd
}
