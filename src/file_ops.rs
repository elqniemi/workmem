use crate::idea::Idea;
use crate::Priority;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn priority_from_str(priority_str: &str) -> Option<Priority> {
    match priority_str {
        "1" => Some(Priority::High),
        "2" => Some(Priority::Medium),
        "3" => Some(Priority::Low),
        _ => None,
    }
}

pub fn read_memory(file_path: &Path) -> io::Result<Vec<Idea>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut ideas = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, ',').collect();
        if parts.len() != 2 {
            continue;
        }

        if let Some(priority) = priority_from_str(parts[0]) {
            let content = parts[1].to_string();
            let idea = Idea { priority, content };
            ideas.push(idea);
        }
    }

    Ok(ideas)
}

pub fn write_memory(file_path: &Path, ideas: &[Idea]) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for idea in ideas {
        writeln!(
            file,
            "{},{}",
            match idea.priority {
                Priority::High => 1,
                Priority::Medium => 2,
                Priority::Low => 3,
            },
            idea.content
        )?;
    }
    Ok(())
}

pub fn append_memory(file_path: &Path, idea: Idea) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

        writeln!(
            file,
            "{},{}",
            match idea.priority {
                Priority::High => 1,
                Priority::Medium => 2,
                Priority::Low => 3,
            },
            idea.content
        )?;

    Ok(())
}

