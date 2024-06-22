use std::fs;

use regex::Regex;

// structure for todo entry
pub struct TodoEntry {
    pub title: String,
    pub done: bool,
    pub index: usize,
}

fn create_todo_entry(todo_dir: &str, title: &str, done: bool, index: usize) {
    let entries = load_todo_entries(todo_dir);
    let index = entries.len() + 1;
    let filename = format!(
        "{}/{}. {} - [{}].md",
        todo_dir,
        index,
        title,
        if done { "Done" } else { "Not Done" }
    );
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
                let entry = TodoEntry { index, title, done };
                entries.push(entry);
            }
        }
    }

    // sort by index number.
    entries.sort_by(|a, b| a.index.cmp(&b.index));

    entries
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

pub fn done_todo() {
    println!("Done todo");
}