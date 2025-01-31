//! Opus types
/// User action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentType {
    /// add a new todo
    Add,
    /// delete a todo 
    Delete,
    /// remove all tasks
    Clear,
    /// mark a todo as finished
    Finish,
    /// list todo matching the query
    List,
    /// given argument is unknown
    Unknown,
    /// not enough arguments supplied
    Notenough,
}

#[derive(Debug)]
pub struct CliInput {
    pub task: Option<Task>,
    pub query: Option<String>,
}

#[derive(Debug)]
pub struct Task {
    pub id: Option<usize>,
    pub title: String,
    pub tag: String,
    pub priority: usize,
    pub due: String,
    pub finished: bool,
}

#[derive(Debug)]
pub struct Cli {
    pub top_level_arg: ArgumentType,
    pub input: CliInput,
}
