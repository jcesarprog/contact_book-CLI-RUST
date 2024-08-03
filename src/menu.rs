use crate::{app::AppState, user::User};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;

#[derive(PartialEq)]
pub enum MenuState {
    MainMenu,
    UserSelectionMenu,
    UserMenu,
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

pub fn main_menu(app: &AppState) -> MenuState {
    show_selected_user_on_prompt(app);
    let items = &["Select a user", "Register a user", "Quit"];
    let selection = create_generic_menu("Choose an option", items);
    match selection {
        0 => MenuState::UserSelectionMenu,
        1 => MenuState::RegisterUser,
        _ => MenuState::Quit,
    }
}

pub fn user_selection_menu<'a>(users: &'a mut Vec<User>, app: &'a mut AppState<'a>) -> MenuState {
    // * SELECTING A USER
    let selected_idx =
        create_generic_menu("Choose an user", &extract_names_from_users(&users, true));
    match selected_idx {
        idx if idx == users.len() => {
            // Back chosen
            println!("Back chosen");
            MenuState::MainMenu
        }
        idx => {
            app.set_user(&mut users[idx]);
            MenuState::UserMenu
        }
    }
}

fn extract_names_from_users(users: &Vec<User>, put_back: bool) -> Vec<&str> {
    let mut names: Vec<&str> = Vec::new();
    for u in users {
        names.push(&u.name);
    }

    if put_back == true {
        names.push("<- Back");
    }
    names
}

pub fn user_menu(app: &AppState) -> MenuState {
    show_selected_user_on_prompt(app);
    let items = ["List contacts", "Edit User", "<- Back"];
    let generic_menu = create_generic_menu(
        "Choose an option related to the current selected user",
        &items,
    );

    match generic_menu {
        0 => MenuState::ListContacts,
        1 => MenuState::EditUserAndContacts,
        2 => MenuState::UserSelectionMenu,
        _ => unreachable!(),
    }
}
