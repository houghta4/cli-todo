//! Printing functions
use crate::Task;
use colored::*;
use std::{io::{self, Write}, process::Command};
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
    // * this is a mess but it works
    let longest_task = tasks.iter().map(|x| x.title.len()).max().unwrap_or(0) + 7 + 8; //len(title + (position + completed) + whitespace)
    let title = "    cli-todo    ";
    let title_offset = if longest_task / 2 >= 8 {
        longest_task / 2 - 8
    } else {
        0
    };
    let box_size: usize = if longest_task > title.len() {
        longest_task
    } else {
        title.len()
    };

    println!("{}", "=".repeat(box_size + 2).on_blue());
    println!(
        "{}{}{}{}{}",
        "|".on_blue(),
        " ".repeat(title_offset),
        title,
        if title_offset== 0 || longest_task % 2 == 0 {
            " ".repeat(title_offset)
        } else {
            " ".repeat(title_offset + 1)
        },
        "|".on_blue()
    );
    println!("{}", "=".repeat(box_size + 2).on_blue());
    println!("{}{}{}", "|".on_blue(), " ".repeat(box_size), "|".on_blue());
    for (i, t) in tasks.iter().enumerate() {
        let extra_space = longest_task - (4 + 7 + 2 + 1) - t.title.len(); // 4 from left whitespace + 7 from len(position + completed box + title) + 2 from bars + 1 default space used in repeat
        println!(
            "{}    {}. [{}] {}   {}{}",
            "|".on_blue(),
            i + 1,
            if t.completed {
                "x".bright_red()
            } else {
                " ".color("")
            },
            t.title.bright_cyan(),
            " ".repeat(extra_space),
            "|".on_blue()
        );
    }
    println!("{}", "=".repeat(box_size + 2).on_blue());
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


pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn().expect("cmd /c cls failed")
            .wait().expect("Failed to wait on command");
    } else {
        Command::new("clear")
            .spawn().expect("clear failed")
            .wait().expect("Failed to wait on command");
    };
}

pub fn print_title() {

    println!("{}",r"
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWMMMMM
MMMMMMMNK0000000000000000000000000000000000000000000000000000000000000000000000000000000000KNWMMMMMM
MMMMMMMk.. ..  .  ...........................'''''',,,,,,;;;;;;::::::cccccccllllllloooooodl:kWMMMMMM
MMMMMMMk.                          ............'''',,,,;;;::::ccllloooodddxxxkkkOOOO000KKXk:kMMMMMMM
MMMMMMMk.                           .............''''',,,;;;:::cccllllooodddxxxkkkOOOO000Kk:kMMMMMMM
MMMMMMMk.                             .............'''',,,;;;:::cccllllooodddxxxkkkkOOOOO0x:kMMMMMMM
MMMMMMMk.     ____   _____       _____           ________    ____    ______      ____000O0x:kMMMMMMM
MMMMMMMk.    / ___) (_   _)     (_   _)         (___  ___)  / __ \  (_  __ \    / __ \kOO0x:kMMMMMMM
MMMMMMMk.   / /       | |         | |   ________    ) )    / /  \ \   ) ) \ \  / /  \ \kkOx:kMMMMMMM
MMMMMMMk.  ( (        | |         | |  (________)  ( (    ( ()  () ) ( (   ) )( ()  () )kOd;MMMMMMMM
MMMMMMMk.  ( (        | |   __    | |               ) )   ( ()  () )  ) )  ) )( ()  () )kOd;kMMMMMMM
MMMMMMMk.   \ \___  __| |___) )  _| |__            ( (     \ \__/ /  / /__/ /  \ \__/ /xxko;kMMMMMMM
MMMMMMMk.    \____) \________/  /_____(            /__\     \____/  (______/    \____/dddxo;kMMMMMMM
MMMMMMMk.                                        .............''',,,,;;;:::ccclllloooddddxo;kMMMMMMM
MMMMMMMk.                                          ............'''',,,;;;::::ccclllooodddxo;kMMMMMMM
MMMMMMMk.  Your todo list is your path to productivity..........'''',,,;;;;:::cccllllooodxl;kMMMMMMM
MMMMMMMk.  Keep your life on track, one task at a time............''',,,,;;;:::cccllllooodl;kMMMMMMM
MMMMMMMk.  Plan your day, plan your tasks, plan your success.......'''',,,;;;::::cccllloodl,kMMMMMMM
MMMMMMMk.                                                ...........'''',,,;;;;:::ccclllooc,kMMMMMMM
MMMMMMMk.                                                 ............''',,,,;;;:::cccclloc,kMMMMMMM
MMMMMMMk.                                                  ............''',,,,;;;:::ccccll:,xMMMMMMM
MMMMMMMk.                                                   .............''',,,;;;;:::cccl:,xMMMMMMM
MMMMMMMk.                                                     ............''',,,,;;;:::ccc:,xMMMMMMM
MMMMMMMk.                                                       ...........'''',,,;;;;:::c:'xMMMMMMM
MMMMMMMk.                                                       ............'''',,,;;;;:::;'xMMMMMMM
MMMMMMMO;''''''''''''''''''''''''''''''''''.............,,,,,,,,,,,;;;;;;;;;;;;;::::::::::::OMMMMMMM
MMMMMMMWNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNKl'''''''''c0NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNWMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWkodddooodokNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWKO00000000KWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWK0000O0000KNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWWNNNNNNNX0OOOOkkkkkOXNNNWWNWWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW0dddddddddddddddoddoooddddddddONMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMXkollllllllloollllllllllllllccldKMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWNNNNNNNNNNNNNNNXXXXXNXXXNNNNNNWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
        ".cyan());
}
