#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Idea {
    pub priority: Priority,
    pub content: String,
}

