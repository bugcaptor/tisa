use regex::Regex;
use std::fs;
use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;

// structure for todo entry
pub struct TodoEntry {
    pub filepath: String,
    pub title: String,
    pub done: bool,
    pub index: usize,
}

fn make_todo_filename(todo_dir: &str, title: &str, done: bool, index: usize) -> String {
    format!(
        "{}/{}. {} - [{}].md",
        todo_dir,
        index,
        title,
        if done { "Done" } else { "Not Done" }
    )
}

fn create_todo_entry(todo_dir: &str, title: &str, done: bool, index: usize) {
    let filename = make_todo_filename(todo_dir, title, done, index);
    fs::write(filename, "").expect("Could not write file");
}

fn load_todo_entries(todo_dir: &str) -> Vec<TodoEntry> {
    let mut entries = Vec::new();
    let paths = fs::read_dir(todo_dir).expect("Could not read directory");

    // 정규 표현식을 컴파일합니다.
    let re = Regex::new(r"(\d+)\. (.+) - \[(Done|Not Done)\]\.md").unwrap();

    for path in paths {
        let path = path.expect("Could not read path").path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            if let Some(captures) = re.captures(filename) {
                let index: usize = captures[1].parse().unwrap();
                let title = captures[2].to_string();
                let done = match captures.get(3) {
                    Some(m) if m.as_str() == "Done" => true,
                    _ => false,
                };
                let entry = TodoEntry {
                    filepath: path.clone().to_str().unwrap().to_string(),
                    index,
                    title,
                    done,
                };
                entries.push(entry);
            }
        }
    }

    // sort by index number.
    entries.sort_by(|a, b| a.index.cmp(&b.index));

    entries
}

fn get_entry_by_index(entries: &Vec<TodoEntry>, index: usize) -> Option<&TodoEntry> {
    for entry in entries {
        if entry.index == index {
            return Some(entry);
        }
    }
    None
}

pub fn add_todo(todo_dir: &str) {
    println!("Adding todo... input todo entry title...");
    // accept standard input text from user.
    let mut todo_entry = String::new();
    std::io::stdin().read_line(&mut todo_entry).unwrap();
    // remove newline character from input text.
    todo_entry = todo_entry.trim().to_string();
    // print the input text.
    println!("Adding todo entry: {}", todo_entry);

    let entries = load_todo_entries(todo_dir);

    // check if entry already exists
    let mut max_index_number = 0;
    for entry in entries {
        if entry.title == todo_entry {
            println!("Already exists: {}", todo_entry);
            return;
        }
        if entry.index > max_index_number {
            max_index_number = entry.index;
        }
    }

    create_todo_entry(todo_dir, &todo_entry, false, max_index_number + 1);
}

pub fn list_todo(todo_dir: &str) {
    let entries = load_todo_entries(todo_dir);
    for entry in entries {
        println!(
            "[{}] {} - {}",
            entry.index,
            entry.title,
            if entry.done { "Done" } else { "Not Done" }
        );
    }
}

pub fn done_todo(todo_dir: &str, index: usize) {
    let entries = load_todo_entries(todo_dir);
    let target_entry = get_entry_by_index(&entries, index);
    match target_entry {
        Some(entry) => {
            if entry.done {
                // set to not done.
                let filename = make_todo_filename(todo_dir, &entry.title, false, entry.index);
                println!("rename: {} -> {}", entry.filepath, filename);
                fs::rename(entry.filepath.clone(), filename).expect("Could not rename file");
                println!("Marked todo entry as not done: {}", entry.title);
            } else {
                let filename = make_todo_filename(todo_dir, &entry.title, true, entry.index);
                println!("rename: {} -> {}", entry.filepath, filename);
                fs::rename(entry.filepath.clone(), filename).expect("Could not rename file");
                println!("Marked todo entry as done: {}", entry.title);
            }
        }
        None => {
            println!("Could not find todo entry with index: {}", index);
            println!("Todo list is here:");
        }
    }
    list_todo(todo_dir);
}

pub fn edit_todo(todo_dir: &str, index: usize) {
    let entries = load_todo_entries(todo_dir);
    let target_entry = get_entry_by_index(&entries, index);
    match target_entry {
        Some(entry) => {
            println!("Editing todo entry: {}", entry.title);
            println!("Input new title...");
            let mut new_title = String::new();
            std::io::stdin().read_line(&mut new_title).unwrap();
            new_title = new_title.trim().to_string();
            println!("New title: {}", new_title);

            let filename = make_todo_filename(todo_dir, &new_title, entry.done, entry.index);
            println!("rename: {} -> {}", entry.filepath, filename);
            fs::rename(entry.filepath.clone(), filename).expect("Could not rename file");
            println!("Edited todo entry: {}", new_title);
        }
        None => {
            println!("Could not find todo entry with index: {}", index);
            println!("Todo list is here:");
        }
    }
    list_todo(todo_dir);
}

pub fn archive_done_todos(todo_dir: &str) {
    let entries = load_todo_entries(todo_dir);

    // 이동 시킬 아카이브 서브 폴더.
    let archive_dir = format!("{}/archive", todo_dir);
    // 없으면 생성합니다.
    let archive_path = std::path::Path::new(&archive_dir);
    if !archive_path.exists() {
        fs::create_dir(&archive_path).expect("Could not create directory");
    }

    for entry in entries {
        if entry.done {
            let filename = make_todo_filename(todo_dir, &entry.title, entry.done, entry.index);
            // move to archive directory.
            let archive_filename =
                make_todo_filename(&archive_dir, &entry.title, entry.done, entry.index);
            // 중복될 수도 있으니까 아카이빙 하는 시간을 붙여줍니다.
            let now = chrono::Local::now();
            let archive_filename =
                archive_filename.replace(".md", &format!("-{}.md", now.format("%Y%m%d%H%M%S")));
            println!("rename: {} -> {}", filename, archive_filename);
            fs::rename(filename, archive_filename).expect("Could not rename file");
        }
    }
    println!("Archived done todos");
}

pub fn search_todos(todo_dir: &str, word: &str) {
    let lower_case_word = word.nfc().collect::<String>().to_lowercase();
    let entries = load_todo_entries(todo_dir);
    for entry in entries {
        let title_normalized = entry.title.nfc().collect::<String>().to_lowercase();
        if title_normalized.contains(&lower_case_word) {
            println!(
                "[{}] {} - {}",
                entry.index,
                entry.title,
                if entry.done { "Done" } else { "Not Done" }
            );
        }
    }
}
