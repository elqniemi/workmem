use crate::file_ops::{append_memory, read_memory, write_memory};
use crate::idea::{Idea, Priority};
use std::path::Path;
use std::io;

const WORKING_MEMORY_FILE: &str = "working_memory.txt";
const HISTORY_FILE: &str = "history.txt";

pub fn add_idea(priority: Priority, content: &str) -> io::Result<()> {
    let idea = Idea {
        priority,
        content: content.to_string(),
    };
    let file_path = Path::new(WORKING_MEMORY_FILE);
    append_memory(&file_path, idea)
}

pub fn list_ideas() -> io::Result<Vec<Idea>> {
    let file_path = Path::new(WORKING_MEMORY_FILE);
    read_memory(&file_path)
}

pub fn save_memory() -> io::Result<()> {
    let workmem_path = Path::new(WORKING_MEMORY_FILE);
    let history_path = Path::new(HISTORY_FILE);

    let ideas = read_memory(&workmem_path)?;

    for idea in ideas {
        append_memory(&history_path, idea)?;
    }

    clear_memory()
}

pub fn clear_memory() -> io::Result<()> {
    let file_path = Path::new(WORKING_MEMORY_FILE);
    write_memory(&file_path, &[])
}

pub fn list_sorted_by_priority() -> io::Result<Vec<Idea>> {
    let mut ideas = list_ideas()?;
    ideas.sort_by(|a, b| b.priority.cmp(&a.priority));
    Ok(ideas)
}

pub fn list_current() -> io::Result<Vec<Idea>> {
    let file_path = Path::new(WORKING_MEMORY_FILE);
    read_memory(&file_path)
}

