use crate::app::AppState;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;

pub enum MenuState {
    MainMenu,
    SelectUserMenu,
    RegisterUser,
    ListContacts,
    EditUserAndContacts,
    Quit,
}

fn clear_prompt() {
    #[cfg(target_os = "windows")]
    Command::new("cls")
        .status()
        .expect("failed to execute process");

    #[cfg(not(target_os = "windows"))]
    Command::new("clear")
        .status()
        .expect("failed to execute process");
}
pub fn create_generic_menu(prompt: &str, items: &[&str]) -> usize {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&items[..])
        .interact()
        .unwrap();

    selection
}

pub fn create_user_menu(app: &AppState) -> (usize, usize) {
    show_selected_user_on_prompt(app);
    let items = ["List contacts", "Edit User", "<- Back"];
    let generic_menu = create_generic_menu(
        "Choose an option related to the current selected user",
        &items,
    );
    (generic_menu, items.len() - 1)
}

fn show_selected_user_on_prompt(app: &AppState) {
    clear_prompt();
    println!("################################");
    println!("Selected: {}", app.get_user().green());
    println!("################################\n");
}

pub fn create_main_menu(app: &AppState) -> usize {
    show_selected_user_on_prompt(app);
    let items = &["Select a user", "Register a user", "Quit"];
    create_generic_menu("Choose an option", items)
}
