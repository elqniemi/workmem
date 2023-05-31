mod idea;
mod file_ops;
mod workmem;

use idea::Priority;
use workmem::{add_idea, list_ideas, save_memory, clear_memory, list_sorted_by_priority};
use std::io::{self, Write};

fn main() {
    loop {
        print!("workmem> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];

        match command {
            "add" => {
                if let Some((priority_str, content)) = parts.get(1).and_then(|s| s.split_once(' ')) {
                    if let Some(priority) = priority_from_str(priority_str) {
                        add_idea(priority, content).unwrap();
                        println!("Idea added.");
                    } else {
                        println!("Invalid priority. Use 1 for High, 2 for Medium, or 3 for Low.");
                    }
                } else {
                    println!("Usage: add <priority> <content>");
                }
            }
            "list" => {
                let ideas = list_ideas().unwrap();
                for (i, idea) in ideas.iter().enumerate() {
                    println!("{}: {:?}, {}", i + 1, idea.priority, idea.content);
                }
            }
            "list_priority" => {
                let ideas = list_sorted_by_priority().unwrap();
                for (i, idea) in ideas.iter().enumerate() {
                    println!("{}: {:?}, {}", i + 1, idea.priority, idea.content);
                }
            }
            "save" => {
                save_memory().unwrap();
                println!("Working memory saved to history.");
            }
            "clear" => {
                clear_memory().unwrap();
                println!("Working memory cleared.");
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Unknown command. Use 'add', 'list', 'list_priority', 'save', 'clear', or 'exit'.");
            }
        }
    }
}

fn priority_from_str(priority_str: &str) -> Option<Priority> {
    match priority_str {
        "1" => Some(Priority::High),
        "2" => Some(Priority::Medium),
        "3" => Some(Priority::Low),
        _ => None,
    }
}

