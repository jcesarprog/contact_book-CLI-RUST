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
    // ## Getting contacts size
    let selected_user = utils::get_selected_user(app, users);
    let contacts_size = match &selected_user.contact {
        Some(contacts) => contacts.len(),
        None => 0,
    };
    let list_contact_opt = format!("List contacts({})", contacts_size);
    // ##

    let selections = &["Edit user", &list_contact_opt, "Add Contact", "<- Back"];

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

pub fn menu_list_contacts_to_select(
    app: &mut AppState,
    users: &HashMap<String, User>,
) -> MenuOption {
    utils::clear_terminal_and_show_user(app, users);
    let current_user = utils::get_selected_user(app, users);
    let contacts = current_user.contact.as_ref().unwrap();
    let mut contact_names = contacts
        .iter()
        .map(|(email, val)| {
            format!(
                "{}\t({})\t{:?}",
                email,
                val.name,
                val.phone
                    .clone()
                    .unwrap_or("No phone registered".to_string())
            )
        })
        .collect::<Vec<_>>();
    let back = "<- Back".to_string();
    contact_names.push(back);
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a contact to edit")
        .default(0)
        .items(&contact_names[..])
        .interact()
        .unwrap();
    if selection == contact_names.len() - 1 {
        return MenuOption::UserMainMenu;
    }
    let contact_selected = contact_names[selection].split_whitespace().next().unwrap();
    app.contact_selected = Some(contact_selected.to_string());
    MenuOption::UserMainMenu
}
