use clap::{arg, Arg, Command};

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
        .subcommand(
            Command::new("todo")
                .about("Manage todo list")
                .subcommand_required(true)
                .subcommand(Command::new("add").about("Add a new todo"))
                .subcommand(Command::new("list").about("List all todos"))
                .subcommand(
                    Command::new("done").about("Mark a todo as done").arg(
                        Arg::new("ID")
                            .help("ID of the todo to mark as done")
                            .required(true)
                            .value_parser(clap::value_parser!(usize)),
                    ),
                )
                .subcommand(
                    Command::new("edit").about("Edit a todo").arg(
                        Arg::new("ID")
                            .help("ID of the todo to edit")
                            .required(true)
                            .value_parser(clap::value_parser!(usize)),
                    ),
                )
                .subcommand(Command::new("archive").about("Archive all done todos"))
                .subcommand(
                    Command::new("search").about("Search for todos").arg(
                        Arg::new("WORD_FOR_SEARCH")
                            .help("Word to search for")
                            .required(true),
                    ),
                ),
        )
}
