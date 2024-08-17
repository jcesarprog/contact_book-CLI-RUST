use std::{collections::HashMap, process::Command};

use colored::Colorize;
use dialoguer::console::{style, Color};

use crate::{app::AppState, menu::MenuOptionAndAction, repo::models::user::User};

pub fn print_with_theme(message: &str) {
    // Use the theme's style to print the message
    // Create a colorful theme and apply a style to the message
    let styled_message = style(message).bold().fg(Color::White); // Example of styling

    println!("{}", styled_message);
}

pub fn show_user(app: &AppState, users: &HashMap<String, User>) {
    let user_selected = match &app.user_selected {
        Some(user_email) => (user_email.as_str(), users[user_email].name.as_str()),
        None => ("No user selected", ""),
    };
    clear_terminal();
    println!("\n###########################################");
    if !user_selected.1.is_empty() {
        println!(
            "### Selected: {} ({})",
            user_selected.0.bright_cyan(),
            user_selected.1.bright_green()
        );
    } else {
        println!("### Selected: {}", user_selected.0.bright_cyan(),);
    }

    println!("###########################################\n");
}

pub fn clear_terminal() {
    // Clear the terminal
    if cfg!(target_os = "windows") {
        Command::new("cls")
            .status()
            .expect("failed to execute process");
    } else {
        Command::new("clear")
            .status()
            .expect("failed to execute process");
    }
}

pub fn clear_terminal_and_show_user(app: &AppState, users: &HashMap<String, User>) {
    clear_terminal();
    show_user(app, users);
    // println!("{:#?}", app);
}

pub fn get_selected_user<'a>(app: &AppState, users: &'a HashMap<String, User>) -> &'a User {
    let user_key = app.user_selected.as_ref().unwrap();
    let selected_user = users.get(user_key).unwrap();
    selected_user
}
pub fn get_selected_user_mut<'a>(
    app: &AppState,
    users: &'a mut HashMap<String, User>,
) -> &'a mut User {
    let user_key = app.user_selected.as_ref().unwrap();
    let selected_user = users.get_mut(user_key).unwrap();
    selected_user
}

pub fn not_implelemted_yet(section: &str) -> MenuOptionAndAction {
    println!("{:?} - Not implemented yet", section);
    MenuOptionAndAction::Quit
}
