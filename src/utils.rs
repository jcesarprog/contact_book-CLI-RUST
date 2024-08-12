use std::process::Command;

use colored::Colorize;
use dialoguer::console::{style, Color};

use crate::app::AppState;

pub fn print_with_theme(message: &str) {
    // Use the theme's style to print the message
    // Create a colorful theme and apply a style to the message
    let styled_message = style(message).bold().fg(Color::White); // Example of styling

    println!("{}", styled_message);
}

pub fn show_user(app: &AppState) {
    let user_selected = match &app.user_selected {
        Some(user_email) => user_email,
        None => &"No user selected".to_string(),
    };
    clear_terminal();
    println!("\n###########################################");
    println!("### Selected: {}", user_selected.bright_cyan());
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

pub fn clear_terminal_and_show_user(app: &AppState) {
    clear_terminal();
    show_user(app)
}
