use std::{
    collections::HashMap,
    fs::File,
    io::{Error, Read},
};

use crate::user::User;

fn load_file(file_name: &str) -> File {
    let file = File::open(file_name);
    match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "There was a error when trying to open file {}\n{}",
                file_name, e
            );
            println!("trying to create the file if the error was due to file not exisiting");
            File::create_new(file_name).unwrap()
        }
    }
}

pub fn load_users_from_json() -> Result<HashMap<String, User>, Error> {
    let mut data = load_file("data.json");
    let mut json_data_string = String::new();
    data.read_to_string(&mut json_data_string)?;

    let users: HashMap<String, User>;
    if json_data_string.is_empty() {
        users = HashMap::new();
        return Ok(users);
    }
    users = serde_json::from_str(&json_data_string).unwrap();
    Ok(users)
}
