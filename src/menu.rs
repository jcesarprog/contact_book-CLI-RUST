use crate::app::AppState;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn create_main_menu(app: &AppState) -> usize {
    println!("################################");
    println!("Selected: {}", app.get_user().green());
    println!("################################\n");

    let items = &["Select a user", "Register a user"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&items[..])
        .interact()
        .unwrap();

    selection
}
