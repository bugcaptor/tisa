mod args;

fn main() {
    let matches = args::build_cli().get_matches();

    match matches.subcommand() {
        Some(("search", sub_matches)) => handle_search(sub_matches),
        _ => unreachable!(),
    }
}

fn handle_search(sub_matches: &clap::ArgMatches) {
    let word = sub_matches.get_one::<String>("WORD_FOR_SEARCH").unwrap();
    println!("Searching for: {}", word);
}
