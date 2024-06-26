mod args;
mod config;
mod folder_walker;
mod note_searcher;
mod todo_note;
mod utility;

use config::{load_config, CONFIG};
use utility::to_absolute_path;

fn main() {
    let matches = args::build_cli().get_matches();
    let config_path = "config.json"; // 설정 파일의 경로를 지정합니다.

    if let Err(e) = load_config(config_path) {
        // make relative path to absolute path
        let config_path = to_absolute_path(&config_path);
        println!("Failed to read config file: {} from {:?}", e, config_path);
        return;
    }

    match matches.subcommand() {
        Some(("search", sub_matches)) => handle_search(sub_matches),
        Some(("todo", sub_matches)) => handle_todo(sub_matches),
        _ => unreachable!(),
    }
}

fn handle_todo(sub_matches: &clap::ArgMatches) {
    let sub_command_name = sub_matches.subcommand_name().unwrap();
    println!("Todo sub command: {}", sub_command_name);

    let config = CONFIG.read().unwrap();

    // switch case
    match sub_matches.subcommand() {
        Some(("add", _sub_sub_matches)) => todo_note::add_todo(config.todo_directory.as_str()),
        Some(("list", _sub_sub_matches)) => todo_note::list_todo(config.todo_directory.as_str()),
        Some(("done", sub_sub_matches)) => todo_note::done_todo(
            config.todo_directory.as_str(),
            *sub_sub_matches.get_one::<usize>("ID").unwrap(),
        ),
        Some(("edit", sub_sub_matches)) => todo_note::edit_todo(
            config.todo_directory.as_str(),
            *sub_sub_matches.get_one::<usize>("ID").unwrap(),
        ),
        Some(("archive", _sub_sub_matches)) => {
            todo_note::archive_done_todos(config.todo_directory.as_str())
        }
        Some(("search", sub_sub_matches)) => todo_note::search_todos(
            config.todo_directory.as_str(),
            sub_sub_matches
                .get_one::<String>("WORD_FOR_SEARCH")
                .unwrap(),
        ),
        _ => println!("Unknown sub command: {}", sub_command_name),
    }
}

fn handle_search(sub_matches: &clap::ArgMatches) {
    let word = sub_matches.get_one::<String>("WORD_FOR_SEARCH").unwrap();
    println!("Searching for: {}", word);

    let config = CONFIG.read().unwrap();
    let dir = &config.directory;
    let pattern = &config.pattern;

    note_searcher::search_note(dir, pattern, &word);
}
