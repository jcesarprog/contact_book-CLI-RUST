use dialoguer::console::{style, Color};

use crate::{app::AppState, user::User};

pub fn print_with_theme(message: &str) {
    // Use the theme's style to print the message
    // Create a colorful theme and apply a style to the message
    let styled_message = style(message).bold().fg(Color::White); // Example of styling

    println!("{}", styled_message);
}

pub fn show_user(app: &AppState, users: &Vec<User>) {
    let user_selected = match app.user_selected {
        Some(user_idx) => &users[user_idx].name,
        None => &"No user selected".to_string(),
    };

    println!("####################",);
    println!("### {} ### ", user_selected);
    println!("####################",);
}
