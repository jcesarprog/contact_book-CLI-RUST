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
        Some(user_email) => &user_email,
        None => &"None".to_string(),
    };

    println!("###########################################");
    println!("### Selected: {}", user_selected);
    println!("###########################################");
}
