//! Functions use to create, read, and write to file
use std::io::Read;
use std::fs::{File, OpenOptions};
use serde_json::Result;
use crate::{Task, User};
/// Write what is stored in `User` obj to file.
/// Overwrites old file data
/// 
/// * `user` - ref of user object to be stored in file
/// 
pub fn write_to_file(user: &User) {
    let _dummy = User {
        name: "Vince".to_string(),
        tasks: vec![Task{ title: "Walk dog".to_string(), completed: false}, Task{ title: "Make dinner".to_string(), completed: true}],
    }; //test value
    let f_name = format!("{}{}", user.name, "_tasks.json");
    let file = OpenOptions::new().create(false).write(true).truncate(true).open(f_name).expect("Error opening file");
    serde_json::to_writer_pretty(&file, &user).expect("Error writing to file");
}



/// If a file does not exist, create one based off username, create an empty `User` obj, and write it to file
/// Else create and populate a `User` obj from file
/// 
/// * `name` - user supplied username used in file creation/access
/// 
/// # Return `User` obj
pub fn populate_user(name: &str) -> Result<User>{
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

    let user: Result<User> = serde_json::from_str(content.as_str());
    user
}