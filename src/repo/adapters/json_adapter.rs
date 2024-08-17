use crate::repo::dao::DAO;
use crate::repo::models::user::User;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, Read};

#[derive(Debug)]
pub struct UserJsonAdapter {
    pub file_path: String,
}

impl DAO for UserJsonAdapter {
    fn get_users(&self) -> Result<HashMap<String, User>, Error> {
        let mut data = load_file(&self.file_path);
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

    fn save_user(&self, users: &HashMap<String, User>) -> Result<(), Error> {
        let users_json_string = turn_users_to_json_string(users)?;
        fs::write(&self.file_path, users_json_string).expect("error saving saving data to file");
        Ok(())
    }
    /*
    fn get_user(&self, key: &str) -> Result<Option<User>, Error> {
        let users = self.get_users().expect("error loading users from json");
        Ok(users.get(key).cloned())
    }

    fn remove_user(&self, key: &str) -> Result<HashMap<String, User>, Error> {
        let mut users = self.get_users().expect("error loading users from json");
        users.remove(key);
        Ok(users)
    }

    fn update_user(&self, key: &str, user: User) -> Result<HashMap<String, User>, Error> {
        let mut users = self.get_users().expect("error loading users from json");
        users.insert(key.to_string(), user);
        Ok(users)
    }
    */
}

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

pub fn turn_users_to_json_string(users: &HashMap<String, User>) -> Result<String, Error> {
    // Serialize it to a JSON string.
    Ok(serde_json::to_string(&users).expect("error turning users data into string!"))
}
