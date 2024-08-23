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

fn strip_ansi_codes(s: &str) -> String {
    // There are some color codes like:
    // \u{1b}[92m
    // skipping from the unicode start until first letter is enconuter
    let mut result = String::new();
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\u{1b}' {
            // Found start of escape sequence..
            // turning on in_scape flag
            in_escape = true;
            continue;
        }
        if in_escape {
            // Everything under here should be skipped
            if c.is_alphabetic() {
                // should be the last letter from escape sequence
                // turn off in_escape flag before continuing
                in_escape = false;
            }
            continue;
        }

        result.push(c);
    }

    result
}

pub fn show_user(app: &AppState, users: &HashMap<String, User>) {
    let user_selected = match &app.user_selected {
        Some(user_email) => (user_email.as_str(), users[user_email].name.as_str()),
        None => ("No user selected", ""),
    };
    clear_terminal();

    let display_content = match !user_selected.1.is_empty() {
        true => format!(
            "{} ({})",
            user_selected.0.bright_cyan(),
            user_selected.1.bright_green()
        ),
        false => format!("{}", user_selected.0.bright_cyan()),
    };

    // Minimum space needed for the content
    let space_for_display_content = strip_ansi_codes(&display_content).chars().count();
    // total width including padding and spaces
    let spaces_on_each_side = 2;
    let hashes_on_each_side = 2;
    let total_width = space_for_display_content + hashes_on_each_side * 2 + spaces_on_each_side * 2;

    // Top border hashes
    println!("\n{:#^1$}", " User ", total_width);

    // Content
    println!(
        "{hashes}{spaces}{content}{spaces}{hashes}",
        hashes = "#".repeat(hashes_on_each_side),
        spaces = " ".repeat(spaces_on_each_side),
        content = display_content,
    );

    // bottom border hashes
    println!("{:#^1$}\n", "", total_width);
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
