use std::io::BufRead;
use serde::{Deserialize, Serialize};
use colored::*;
mod printing;
mod validation;
mod storage;

// region: data-structures

/// A `User` is used to encapsulate all data relevant to the client
/// This is what will be saved/loaded from file
/// 
/// * `name` - String that will determine the name of the file to create/read/write
/// * `tasks` - Vector of `Task`s in the todo list related to the user
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    name: String,
    tasks: Vec<Task>
}

/// A `Task` is used to represent an entry in a list of todos 
/// 
/// * `title` - String containing the title of the task
/// * `completed` - bool flag representing if the task is completed or not
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    title: String,
    completed: bool,

}

/// Enumerates all possible `Command` options
enum Command {
    /// Add a task to your todos
    Add(Task),
    /// Update a task's title by its position in the list
    Update(usize, String),
    /// Remove a task by its position in the list
    Remove(usize),
    /// Remove ALL tasks
    Clear,
    /// Complete a task by its position in the list
    /// Can uncomplete a task by invoking on a completed task
    Complete(usize),
    /// Undo a given amount of commands. 
    /// Does NOT reset the cache after an undo, so you can undo your undos
    Undo(usize),
    // Swap(usize, usize), // TODO: 'swap 1 2' moves task 1 into position 2 and task 2 into position 1 
    // Search(&str), // TODO: 'search banana' prints task position
    // Move(usize, usize) // TODO: 'move 4 2' moves task 4 to position 2, bumping task 2 to 3, 3 to 4, etc
    /// Provides a list of all commands with an example use case
    Help,
    /// Quits the app and saves to file
    Quit,
}

impl Command {
    /// Determines what command is run based off of user input
    /// 
    /// * `s` - str slice of user input
    /// 
    /// # Returns Option\<Command\>
    fn op(s: &str) -> Option<Self> {
        // split s into ["<Command>", "<args>"]
        let c: Vec<&str> = s.trim().splitn(2, ' ').collect();

        match c.as_slice() {
            ["add", t] => Some(Command::Add(Task { title: t.to_string(), completed: false })),
            ["update", args] => {
                let x: Vec<&str> = args.splitn(2, ' ').collect();
                if x.len() == 2 {
                    Some(Command::Update(x[0].parse::<usize>().unwrap_or(usize::MAX), x[1].to_string()))
                } else {
                    None
                } 
            }
            ["remove", t_id] => Some(Command::Remove(t_id.parse::<usize>().unwrap_or(usize::MAX))),
            ["clear"] => Some(Command::Clear),
            ["complete", t_id] => Some(Command::Complete(t_id.parse::<usize>().unwrap_or(usize::MAX))),
            ["undo", steps] => Some(Command::Undo(steps.parse::<usize>().unwrap_or(usize::MAX))),
            ["help"] | ["help" , ..] => Some(Command::Help),
            ["quit"] | ["quit", ..] => Some(Command::Quit),
            _ => None
        }

    }
}

// endregion: data-structures

fn main() {
    // * get username
    println!("Enter username: ");
    printing::print_prompt_symbol(">");
    let username = std::io::stdin().lock().lines().next().expect("Error reading username").unwrap_or("Default".to_string());

    // * initialize cache and user
    let mut user_cache: Vec<User> = vec![]; // every action will push a user into this
    let mut user: User = storage::populate_user(&username).expect("Error reading User from file");

    printing::print_tasks(&user.tasks);
    println!("{}", "Type 'help' to see instructions available".bright_yellow());
    printing::print_prompt_symbol(">");


    // * main loop
    for line in std::io::stdin().lock().lines() {
        let input: String = line.expect("Error reading input");

        // * Do what is needed based off of user supplied input
        match Command::op(&input) {
            Some(Command::Add(task)) => {
                user_cache.push(user.clone());
                user.tasks.push(task);
            }
            Some(Command::Update(t_id, title)) => {
                if validation::is_valid_task_num(t_id, user.tasks.len(), "Type 'update <number of task> <new title>' to update the name of the task") {
                    user_cache.push(user.clone());
                    user.tasks[t_id - 1].title = title;
                }
            }
            Some(Command::Complete(t_id)) => {
                if validation::is_valid_task_num(t_id, user.tasks.len(), "Type 'complete <number of task>' to complete status of the task") {
                    user_cache.push(user.clone());
                    let mut c = &mut user.tasks[t_id - 1];
                    c.completed = !c.completed
                }
            }
            Some(Command::Remove(t_id)) => {
                if validation::is_valid_task_num(t_id, user.tasks.len(), "Type 'remove <number of task>' to complete status of the task") {
                    user_cache.push(user.clone());
                    user.tasks.remove(t_id - 1);
                }
            }
            Some(Command::Undo(steps)) => {
                if validation::is_valid_undo(steps, user_cache.len(), "Type 'undo <number of commands>' to undo a given amount of commands.\nMust be < total commands in current session") {
                    user_cache.push(user.clone());
                    // set user to correct index of user_cache
                    let cache_size = user_cache.len();
                    user = user_cache[cache_size - steps - 1].clone();
                }
                
            }
            Some(Command::Clear) => {
                user_cache.push(user.clone());
                user.tasks.clear()
            }
            Some(Command::Help) => printing::print_instr(),
            Some(Command::Quit) => break,
            _ => println!("{}", "Invalid command. Type 'help' to see command options.".bright_red())
        }
        storage::write_to_file(&user);
        printing::print_tasks(&user.tasks);
        printing::print_prompt_symbol(">");
    }

    storage::write_to_file(&user);
    println!("Bye!\nThanks for using cli-todo!");
}   
