use dialoguer::{theme::ColorfulTheme, Input};

use crate::utils;

use super::contact::Contact;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub contact: Option<Vec<Contact>>,
}

impl User {
    pub fn new() -> Self {
        Self {
            name: {
                utils::print_with_theme("User details:");
                set_input("User name:")
            },
            email: set_email(),
            contact: None,
        }
    }
}

fn set_input(prompt: &str) -> String {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap();

    println!("Hello {}!", input);
    input
}

fn set_email() -> String {
    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("email:")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
                    Ok(())
                } else {
                    force = Some(input.clone());
                    Err("This is not a mail address; type the same value again to force use")
                }
            }
        })
        .interact_text()
        .unwrap();

    println!("Email: {}", mail);
    mail
}
