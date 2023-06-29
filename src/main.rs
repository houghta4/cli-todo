use std::io::{self, BufRead, Write, Read};
use std::fs::{File, OpenOptions};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use colored::*;

/// Print a symbol with a following whitespace to stdout without a new line and flush the buffer
/// * `sym` - str slice to be printed
/// # Examples
/// ```
/// print_prompt_symbol("$");
/// ``` 
fn print_prompt_symbol(sym: &str) {
    print!("{} ", sym);
    io::stdout().flush().unwrap();
}

/// Print and format tasks to terminal
/// 
///* `tasks` - ref to vector containing tasks to display
fn print_tasks(tasks: &Vec<Task>) {
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
fn print_instr() {
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

/// A `User` is used to encapsulate all data relevant to the client
/// This is what will be saved/loaded from file
/// 
/// * `name` - String that will determine the name of the file to create/read/write
/// * `tasks` - Vector of `Task`s in the todo list related to the user
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    name: String,
    tasks: Vec<Task>
}

/// A `Task` is used to represent an entry in a list of todos 
/// 
/// * `title` - String containing the title of the task
/// * `completed` - bool flag representing if the task is completed or not
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Task {
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
    /// Provides a list of all commands
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

/// Write what is stored in `User` obj to file.
/// Overwrites old file data
/// 
/// * `user` - ref of user object to be stored in file
/// 
fn write_to_file(user: &User) {
    let _dummy = User {
        name: "Vince".to_string(),
        tasks: vec![Task{ title: "Walk dog".to_string(), completed: false}, Task{ title: "Make dinner".to_string(), completed: true}],
    }; //test value
    let f_name = format!("{}{}", user.name, "_tasks.json");
    let file = OpenOptions::new().create(false).write(true).truncate(true).open(&f_name).expect("Error opening file");
    serde_json::to_writer_pretty(&file, &user).expect("Error writing to file");
}



/// If a file does not exist, create one based off username, create an empty `User` obj, and write it to file
/// Else create and populate a `User` obj from file
/// 
/// * `name` - user supplied username used in file creation/access
/// 
/// # Return `User` obj
fn populate_user(name: &str) -> Result<User>{
    let f_name = format!("{}{}", name, "_tasks.json");
    let mut file = match File::open(&f_name) {
        Ok(f) => f,
        // file does not exist
        _ => {
            let f = File::create(&f_name).expect("Failed to create file");
            let user = User { name: name.to_string(), tasks: Vec::new()};
            serde_json::to_writer_pretty(&f, &user).expect("Error writing to file");
            File::open(&f_name).expect("Error opening new file")
        }
    };
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not deserialize file");

    let user: Result<User> = serde_json::from_str(&content.as_str());
    user
}

/// Check to see if given task id is within bounds
/// 
/// * `t_id` - task id 
/// * `len` - length of current todo list
/// * `msg` - error message printed if task id is out of bounds
/// 
fn is_valid_task_num(t_id: usize, len: usize, msg: &str) -> bool {
    match t_id {
        _i if len == 0 => println!("{}", "Task list empty. Add a task before trying to change anything!".bright_red()),
        i if i <= 0 || i > len => {
            println!("{} {}", "Invalid argument:".bright_red(), i);
            println!("{}", msg.bright_red());
        }
        _ => return true 
    }
    false
}

/// Check to see if undo can step back a given amount of commands
/// 
/// * `steps` - amount of steps to undo
/// * `len` - length of commands made during session
/// * `msg` - error message printed if `steps` is invalid
/// 
fn is_valid_undo(steps: usize, len: usize, msg: &str) -> bool {
    match steps {
        _i if len == 0 => println!("{}", "Nothing to undo. Must use a command before undoing".bright_red()),
        i if i <= 0 || i > len => {
            println!("{} {}", "Invalid argument:".bright_red(), i);
            println!("{}", msg.bright_red());
        }
        _ => return true 
    }
    false
}


fn main() {
    // * get username
    println!("Enter username: ");
    print_prompt_symbol(">");
    let username = std::io::stdin().lock().lines().next().expect("Error reading username").unwrap_or("Default".to_string());

    // * initialize cache and user
    let mut user_cache: Vec<User> = vec![]; // every action will push a user into this
    let mut user: User = populate_user(&username).expect("Error reading User from file");

    print_tasks(&user.tasks);
    println!("{}", "Type 'help' to see instructions available".bright_yellow());
    print_prompt_symbol(">");


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
                if is_valid_task_num(t_id, user.tasks.len(), "Type 'update <number of task> <new title>' to update the name of the task") {
                    user_cache.push(user.clone());
                    user.tasks[t_id - 1].title = title;
                }
            }
            Some(Command::Complete(t_id)) => {
                if is_valid_task_num(t_id, user.tasks.len(), "Type 'complete <number of task>' to complete status of the task") {
                    user_cache.push(user.clone());
                    let mut c = &mut user.tasks[t_id - 1];
                    c.completed = !c.completed
                }
            }
            Some(Command::Remove(t_id)) => {
                if is_valid_task_num(t_id, user.tasks.len(), "Type 'remove <number of task>' to complete status of the task") {
                    user_cache.push(user.clone());
                    user.tasks.remove(t_id - 1);
                }
            }
            Some(Command::Undo(steps)) => {
                if is_valid_undo(steps, user_cache.len(), "Type 'undo <number of commands>' to undo a given amount of commands.\nMust be < total commands in current session") {
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
            Some(Command::Help) => print_instr(),
            Some(Command::Quit) => break,
            _ => println!("{}", "Invalid command. Type 'help' to see command options.".bright_red())
        }
        write_to_file(&user);
        print_tasks(&user.tasks);
        print_prompt_symbol(">");
    }

    write_to_file(&user);
    println!("Bye!\nThanks for using cli-todo!");
}   
