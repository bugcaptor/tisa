use std::path::Path;

use crate::folder_walker::find_files_by_pattern;

pub fn search_note(root_dir: &str, pattern: &str, word: &str) {
    let mut total_searched_files = 0;
    let mut previous_folder_name = String::new();
    find_files_by_pattern(root_dir, pattern)
        .unwrap()
        .iter()
        .for_each(|filename| {
            // search word in file, line by line, and print line number and count
            let content = std::fs::read_to_string(filename).unwrap();
            let mut line_number = 0;
            let mut line_numbers_with_word = Vec::new();
            for line in content.lines() {
                line_number += 1;
                if line.contains(&*word) {
                    //println!("{}:{}: {}", f, line_number, line);
                    line_numbers_with_word.push(line_number);
                    break; // 하나만 찾는다 일단.
                }
            }

            // 탐색 결과 내용이 일치하는 게 없으면 이 파일은 아무 것도 출력하지 않습니다.
            if line_numbers_with_word.len() == 0 {
                return;
            }

            total_searched_files += 1;
            //println!("{}: {} lines found", filename, line_numbers_with_word.len());

            // extract folder path string from filename variable.
            let folder_name =
                remove_filename_from_path(get_relative_path(&filename, &root_dir).as_str());
            //println!("folder name: {}", folder_name);
            // if folder name is different from previous folder name, print folder name.
            if folder_name != previous_folder_name {
                println!("- {}", folder_name);
                previous_folder_name = folder_name.to_string();
            }
            // print filename only without directory name.
            println!(
                "  - {}",
                filename.split("/").collect::<Vec<&str>>().last().unwrap()
            );
        });
}

fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let root_dir = Path::new(&root_dir);
    let file_path = Path::new(&file_path);
    let relative_path = file_path.strip_prefix(root_dir).unwrap();
    relative_path.to_str().unwrap().to_string() // convert &Path to String;
}

fn remove_filename_from_path(file_path: &str) -> String {
    let file_path = Path::new(&file_path);
    let parent_path = file_path.parent().unwrap();
    parent_path.to_str().unwrap().to_string()
}
