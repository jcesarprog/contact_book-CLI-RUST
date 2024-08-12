use dialoguer::{theme::ColorfulTheme, Input};

use crate::utils;

#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

impl Contact {
    pub fn new() -> Self {
        Self {
            name: {
                utils::print_with_theme("Contact details:");
                set_input("Contact name:")
            },
            email: set_email(),
            phone: None,
        }
    }
}

fn set_input(prompt: &str) -> String {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap();
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
    mail
}
