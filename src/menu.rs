use std::collections::HashMap;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::{app::AppState, user::User, utils};

#[derive(Debug)]
pub enum MenuOption {
    MainMenu,          // Menu with select and register user
    ListUsersToSelect, // Menu with the list of users to select
    UserMainMenu,      // Menu with the options for a selected user
    Quit,
    RegisterUser,
    EditUser,
    ListContacts,
    AddContact,
}

pub fn menu_select_register_user() -> MenuOption {
    let selections = &["Select a user", "Register a user", "Quit"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => MenuOption::ListUsersToSelect,
        1 => MenuOption::RegisterUser,
        _ => MenuOption::Quit,
    }
}

pub fn menu_list_users_to_select(app: &mut AppState, users: &HashMap<String, User>) -> MenuOption {
    utils::clear_terminal_and_show_user(app, users);
    let mut user_names: Vec<&String> = users.keys().collect();
    let back = "<- Back".to_string();
    user_names.push(&back);
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an user")
        .default(0)
        .items(&user_names[..])
        .interact()
        .unwrap();
    if selection == user_names.len() - 1 {
        return MenuOption::MainMenu;
    }

    app.user_selected = Some(user_names[selection].clone());
    MenuOption::UserMainMenu
}

pub fn menu_user_menu(app: &AppState, users: &HashMap<String, User>) -> MenuOption {
    utils::clear_terminal_and_show_user(app, users);

    let selections = &["Edit user", "List contacts", "Add Contact", "<- Back"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => MenuOption::EditUser,
        1 => MenuOption::ListContacts,
        2 => MenuOption::AddContact,
        _ => MenuOption::ListUsersToSelect,
    }
}
