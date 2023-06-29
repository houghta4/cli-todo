//! Printing functions
use std::io::{self, Write};
use colored::*;
use crate::Task;
/// Print a symbol with a following whitespace to stdout without a new line and flush the buffer
/// * `sym` - str slice to be printed
/// # Examples
/// ```
/// print_prompt_symbol("$");
/// ``` 
pub fn print_prompt_symbol(sym: &str) {
    print!("{} ", sym);
    io::stdout().flush().unwrap();
}

/// Print and format tasks to terminal
/// 
///* `tasks` - ref to vector containing tasks to display
pub fn print_tasks(tasks: &Vec<Task>) {
    // TODO: make pretty
    let _longest_task_title = tasks.iter().map(|x| x.title.len()).max().unwrap_or(0); // used for wall offset when boxing in tasks

    println!();
    println!("{}", "=".repeat(26).on_blue());
    println!("{}{}{}","|".on_blue(), "\t cli-todo \t ", "|".on_blue());
    println!("{}", "=".repeat(26).on_blue());
    println!("{}{}{}","|".on_blue(), " ".repeat(24), "|".on_blue());
    for (i, t) in tasks.iter().enumerate() {
        println!("{}\t{}. [{}] {}\t{}", "|".on_blue(), i + 1, if t.completed {"x".bright_red()} else {" ".color("")}, t.title.bright_cyan(), "|".on_blue());
    }
    println!("{}", "=".repeat(26).on_blue());
    println!();
}

/// Print instructions to stdout
pub fn print_instr() {
    println!("{}", "Type 'add <title of task> to add a new task".bright_yellow());
    println!("{}", "Type 'update <number of task> <new title>' to update the name of the task".bright_yellow());
    println!("{}", "Type 'remove <title of task>' to remove task".bright_yellow());
    println!("{}", "Type 'clear' to remove all tasks".bright_yellow());
    println!("{}", "Type 'complete <number of task>' to update the task as completed or uncompleted".bright_yellow());
    println!("{}", "Type 'undo <number of commands>' to undo a given amount of commands".bright_yellow());
    println!("{}", "Type 'help' to see instructions again".bright_yellow());
    println!("{}", "Type 'quit' to quit and save".bright_yellow());
    println!("{}", "(Case sensivite)".bright_yellow());
    println!();
}
