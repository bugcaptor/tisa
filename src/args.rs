use clap::{arg, Command};

pub fn build_cli() -> Command {
    Command::new("tisa")
        .about("Tisa is a simple cli agent for management of notes.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("search")
                .about("Search for notes")
                .arg(arg!(<WORD_FOR_SEARCH> "Word to search for"))
                .arg_required_else_help(true),
        )
}
