mod args;
mod config;
mod folder_walker;

use std::{env, path::PathBuf};

use config::{load_config, CONFIG};
use folder_walker::find_files_by_pattern;

fn to_absolute_path(relative_path: &str) -> std::io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let absolute_path = current_dir.join(relative_path);
    Ok(absolute_path)
}

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
        _ => unreachable!(),
    }
}

fn handle_search(sub_matches: &clap::ArgMatches) {
    let word = sub_matches.get_one::<String>("WORD_FOR_SEARCH").unwrap();
    println!("Searching for: {}", word);

    let config = CONFIG.read().unwrap();
    let dir = &config.directory;
    let pattern = &config.pattern;

    let mut total_searched_files = 0;
    find_files_by_pattern(dir, pattern)
        .unwrap()
        .iter()
        .for_each(|f| {
            //println!("{}", f)
            // search word in file, line by line, and print line number and count
            let content = std::fs::read_to_string(f).unwrap();
            let mut line_number = 0;
            let mut line_numbers_with_word = Vec::new();
            for line in content.lines() {
                line_number += 1;
                if line.contains(&*word) {
                    //println!("{}:{}: {}", f, line_number, line);
                    line_numbers_with_word.push(line_number);
                }
            }
            // print summary of searching, count of lines containing the word.
            if (line_numbers_with_word.len() > 0) {
                total_searched_files += 1;
                println!("{}: {} lines found", f, line_numbers_with_word.len());
            }
        });

    println!("{} files found", total_searched_files);
}
