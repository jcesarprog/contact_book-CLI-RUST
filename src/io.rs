use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Error, Read},
};

use crate::repo::models::user::User;

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

pub fn load_data_from_json(file_name: &str) -> Result<HashMap<String, User>, Error> {
    let mut data = load_file(file_name);
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

pub fn save_data_to_json(users: &HashMap<String, User>, file_name: &str) -> Result<(), Error> {
    let users_json_string = turn_users_to_string(users)?;
    fs::write(file_name, users_json_string).expect("error saving saving data to file");
    Ok(())
}

pub fn turn_users_to_string(users: &HashMap<String, User>) -> Result<String, Error> {
    // Serialize it to a JSON string.
    Ok(serde_json::to_string(&users).expect("error turning users data into string!"))
}

pub fn save_data_to_json_and_reload_data_in_memory(
    users: &HashMap<String, User>,
    file_name: &str,
) -> Result<HashMap<String, User>, Error> {
    save_data_to_json(&users, file_name)?;
    load_data_from_json(file_name)
}
