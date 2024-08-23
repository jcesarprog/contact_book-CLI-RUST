use std::collections::HashMap;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    app::AppState,
    repo::{
        dao::DAO,
        models::{contact::Contact, user::User},
    },
    utils,
};

#[derive(Debug)]
pub enum MenuOptionAndAction {
    MainMenu,          // Menu with select and register user
    ListUsersToSelect, // Menu with the list of users to select
    UserMainMenu,      // Menu with the options for a selected user
    Quit,
    RegisterUser,
    EditUser,
    ListContacts, // Menu with the list of contacts
    AddContact,
    EditContact,
    RemoveUser,
    ContactOptions,
    RemoveContact,
}

pub fn menu_select_register_user() -> MenuOptionAndAction {
    let selections = &["Select a user", "Quit"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => MenuOptionAndAction::ListUsersToSelect,
        _ => MenuOptionAndAction::Quit,
    }
}

pub fn menu_list_users_to_select(
    app: &mut AppState,
    users: &HashMap<String, User>,
) -> MenuOptionAndAction {
    utils::clear_terminal_and_show_user(app, users);
    let mut user_names: Vec<&String> = users.keys().collect();
    let register_user = "Register a user".to_string();
    let back = "<- Back".to_string();
    user_names.push(&register_user);
    user_names.push(&back);
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an user")
        .default(0)
        .items(&user_names[..])
        .interact()
        .unwrap();

    match selection {
        sel if sel == user_names.len() - 2 => MenuOptionAndAction::RegisterUser,
        sel if sel == user_names.len() - 1 => {
            app.user_selected = None;
            MenuOptionAndAction::MainMenu
        }
        _ => {
            app.user_selected = Some(user_names[selection].clone());
            MenuOptionAndAction::UserMainMenu
        }
    }
}

pub fn menu_user_menu(app: &mut AppState, users: &HashMap<String, User>) -> MenuOptionAndAction {
    utils::clear_terminal_and_show_user(app, users);
    // ## Getting contacts size
    let selected_user = utils::get_selected_user(app, users);
    let contacts_size = match &selected_user.contact {
        Some(contacts) => contacts.len(),
        None => 0,
    };
    let list_contact_opt = format!("List contacts({})", contacts_size);
    // ##

    let selections = &["Edit user", &list_contact_opt, "Remove User", "<- Back"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => MenuOptionAndAction::EditUser,
        1 => MenuOptionAndAction::ListContacts,
        2 => MenuOptionAndAction::RemoveUser,
        _ => {
            app.user_selected = None;
            MenuOptionAndAction::ListUsersToSelect
        }
    }
}

pub fn menu_list_contacts_to_select(
    app: &mut AppState,
    users: &HashMap<String, User>,
) -> MenuOptionAndAction {
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
            let add_contact_opt = "Add Contact".to_string();
            let back_opt = "<- Back".to_string();
            contact_names.push(add_contact_opt);
            contact_names.push(back_opt);

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose a contact to edit")
                .default(0)
                .items(&contact_names[..])
                .interact()
                .unwrap();

            match selection {
                len if len == contact_names.len() - 2 => MenuOptionAndAction::AddContact,
                len if len == contact_names.len() - 1 => MenuOptionAndAction::UserMainMenu,
                _ => {
                    let contact_selected =
                        contact_names[selection].split_whitespace().next().unwrap();
                    app.contact_selected = Some(contact_selected.to_string());
                    MenuOptionAndAction::ContactOptions
                }
            }
        }
        None => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose a contact to edit")
                .default(0)
                .items(&["Add Contact", "<- Back"])
                .interact()
                .unwrap();
            match selection {
                0 => MenuOptionAndAction::AddContact,
                1 => MenuOptionAndAction::UserMainMenu,
                _ => unreachable!(),
            }
        }
    }
}

pub fn menu_register_user(
    app: &mut AppState,
    users: &mut HashMap<String, User>,
    adapter_dao: &impl DAO,
) -> MenuOptionAndAction {
    // create the user
    let u = User::new();
    // set the user to be selected on state
    app.user_selected = Some(u.email.clone());
    // insert the user on the users vector
    users.insert(u.email.clone(), u);
    adapter_dao
        .save_user(users)
        .expect("error saving data to json");

    MenuOptionAndAction::UserMainMenu
}

pub fn menu_edit_user(
    app: &mut AppState,
    users: &mut HashMap<String, User>,
) -> MenuOptionAndAction {
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

    MenuOptionAndAction::UserMainMenu
}

pub fn menu_add_contact(
    app: &mut AppState,
    users: &mut HashMap<String, User>,
) -> MenuOptionAndAction {
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
    MenuOptionAndAction::ListContacts
}

pub fn menu_edit_contact(
    app: &mut AppState,
    users: &mut HashMap<String, User>,
) -> MenuOptionAndAction {
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
    MenuOptionAndAction::ListContacts
}

pub fn menu_remove_user(
    app: &mut AppState,
    users: HashMap<String, User>,
    adapter_dao: &impl DAO,
) -> HashMap<String, User> {
    let new_users = adapter_dao
        .remove_user(app.user_selected.as_ref().unwrap(), users)
        .expect("error removing the user");
    app.user_selected = None;
    new_users
}

pub fn menu_contact_options(app: &AppState, users: &HashMap<String, User>) -> MenuOptionAndAction {
    utils::clear_terminal_and_show_user(app, users);
    let options = ["Edit", "Remove", "Back"];
    let prompt = format!(
        "Choose an action over contact: {:?} ",
        &app.contact_selected.as_ref().unwrap()
    );
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt(&prompt)
        .default(0)
        .items(&options)
        .interact()
        .unwrap()
    {
        0 => MenuOptionAndAction::EditContact,
        1 => MenuOptionAndAction::RemoveContact,
        _ => MenuOptionAndAction::ListContacts,
    }
}
