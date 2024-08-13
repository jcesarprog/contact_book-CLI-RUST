use std::collections::HashMap;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::{app::AppState, contact::Contact, user::User, utils};

#[derive(Debug)]
pub enum MenuOption {
    MainMenu,          // Menu with select and register user
    ListUsersToSelect, // Menu with the list of users to select
    UserMainMenu,      // Menu with the options for a selected user
    Quit,
    RegisterUser,
    EditUser,
    ListContacts, // Menu with the list of contacts
    AddContact,
    EditContact,
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
    let contacts = current_user.contact.as_ref();
    match contacts {
        Some(contacts) => {
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
            MenuOption::EditContact
        }
        None => {
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose a contact to edit")
                .default(0)
                .items(&["<- Back"])
                .interact()
                .unwrap();

            MenuOption::UserMainMenu
        }
    }
}

pub fn menu_register_user(app: &mut AppState, users: &mut HashMap<String, User>) -> MenuOption {
    // create the user
    let u = User::new();
    // set the user to be selected on state
    app.user_selected = Some(u.email.clone());
    // insert the user on the users vector
    users.insert(u.email.clone(), u);
    MenuOption::UserMainMenu
}

pub fn menu_edit_user(app: &mut AppState, users: &mut HashMap<String, User>) -> MenuOption {
    // This option will clone the user contacts
    // delete the original key and create a new one with the same old contacs
    // 1. Creating a new user
    let mut u: User = User::new();
    // 2. Retrieving old data
    let former_user_str = app.user_selected.clone().unwrap();
    let former_user = users.remove(&former_user_str).unwrap();
    // 3. add old contacts to new user
    if let Some(former_contacts) = former_user.contact {
        u.contact = Some(former_contacts.clone());
    }
    // 4. update the state
    app.user_selected = Some(u.email.clone());
    // 5. insert the new user to the map
    users.insert(u.email.clone(), u);

    MenuOption::UserMainMenu
}

pub fn menu_add_contact(app: &mut AppState, users: &mut HashMap<String, User>) -> MenuOption {
    let selected_user = utils::get_selected_user_mut(app, users);
    let new_contact = Contact::new();
    match &mut selected_user.contact {
        None => {
            selected_user.contact = Some(HashMap::from([(new_contact.email.clone(), new_contact)]));
        }
        Some(contacts) => {
            contacts.insert(new_contact.email.clone(), new_contact);
        }
    }
    MenuOption::UserMainMenu
}

pub fn menu_edit_contact(app: &mut AppState, users: &mut HashMap<String, User>) -> MenuOption {
    // 1. get the selected user
    let selected_user = utils::get_selected_user_mut(app, users);
    // 2. remove the selected contact
    selected_user
        .contact
        .as_mut()
        .unwrap()
        .remove(app.contact_selected.as_ref().unwrap());
    app.contact_selected = None;
    // 3. create a new contact and replace the former
    let new_contact = Contact::new();
    selected_user
        .contact
        .as_mut()
        .unwrap()
        .insert(new_contact.name.clone(), new_contact);
    MenuOption::ListContacts
}
