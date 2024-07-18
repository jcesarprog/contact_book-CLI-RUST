use serde::{Deserialize, Serialize};
use std::io;
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub contacts: Option<Vec<User>>,
}
impl User {
    pub fn new() -> Self {
        let mut name = String::new();
        let mut email = String::new();
        let mut phone = String::new();
        ask_for_and_set("name", &mut name);
        ask_for_and_set("email", &mut email);
        ask_for_and_set("phone", &mut phone);
        User {
            name: name.trim().to_string(),
            email: email.trim().to_string(),
            phone: Some(phone.trim().to_string()),
            contacts: None,
        }
    }
    pub fn new_with_data(name: String, email: String, phone: String) -> Self {
        User {
            name: name.trim().to_string(),
            email: email.trim().to_string(),
            phone: Some(phone.trim().to_string()),
            contacts: None,
        }
    }
    pub fn add_contact(&mut self) {
        let mut name = String::new();
        let mut email = String::new();
        let mut phone = String::new();
        ask_for_and_set("name", &mut name);
        ask_for_and_set("email", &mut email);
        ask_for_and_set("phone", &mut phone);
        let u = User {
            name: name.trim().to_string(),
            email: email.trim().to_string(),
            phone: Some(phone.trim().to_string()),
            contacts: None,
        };
        self.contacts = Some(vec![u]);
    }
}
fn ask_for_and_set(field: &str, concrete_field: &mut String) {
    println!("\nWrite your {field}:");
    io::stdin()
        .read_line(concrete_field)
        .expect("Error parsing name");
}
