pub mod cb_io {
    use crate::cb_user::cb_user::User;
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Write},
    };
    pub fn save_data(data: &Vec<User>, file_name: &str) {
        let ser = serde_json::to_string_pretty(data).expect("Error serializing data");
        // println!("{}", ser)

        // ! creating the JSON file with the json data
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
            .expect("Error creating / opening file users.json");

        file.write_all(ser.as_bytes())
            .expect("Error writing JSON to file")
    }

    pub fn read_data(file_name: &str) -> Vec<User> {
        let mut file = File::open(file_name).expect("Error opening file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error parsing json data as string into contents");

        let users: Vec<User> =
            serde_json::from_str(&contents).expect("error parsing json data to Vec<User>");

        users
    }
}
