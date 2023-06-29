//! Functions used to validate various commands
use colored::*;
/// Check to see if given task id is within bounds
/// 
/// * `t_id` - task id 
/// * `len` - length of current todo list
/// * `msg` - error message printed if task id is out of bounds
/// 
pub fn is_valid_task_num(t_id: usize, len: usize, msg: &str) -> bool {
    match t_id {
        _i if len == 0 => println!("{}", "Task list empty. Add a task before trying to change anything!".bright_red()),
        i if i <= 0 || i > len => {
            println!("{}", "Invalid argument".bright_red());
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
pub fn is_valid_undo(steps: usize, len: usize, msg: &str) -> bool {
    match steps {
        _i if len == 0 => println!("{}", "Nothing to undo. Must use a command before undoing".bright_red()),
        i if i <= 0 || i > len => {
            println!("{}", "Invalid argument:".bright_red());
            println!("{}", msg.bright_red());
        }
        _ => return true 
    }
    false
}
