mod app;
mod contact;
mod io;
mod menu;
mod user;
mod utils;

use std::{collections::HashMap, process::exit};

use app::AppState;
use contact::Contact;
use menu::MenuOption;
use user::User;

fn main() {
    let mut app = AppState::new();
    let mut users = io::load_users().expect("Error loading users");

    loop {
        app.menu_state = match app.menu_state {
            MenuOption::MainMenu => {
                utils::clear_terminal_and_show_user(&app, &users);
                menu::menu_select_register_user()
            }
            MenuOption::RegisterUser => {
                // create the user
                let u = User::new();
                // set the user to be selected on state
                app.user_selected = Some(u.email.clone());
                // insert the user on the users vector
                users.insert(u.email.clone(), u);
                MenuOption::UserMainMenu
            }
            MenuOption::ListUsersToSelect => menu::menu_list_users_to_select(&mut app, &users),

            MenuOption::UserMainMenu => menu::menu_user_menu(&app, &users),
            MenuOption::EditUser => {
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
            MenuOption::AddContact => {
                let selected_user = utils::get_selected_user_mut(&app, &mut users);
                let new_contact = Contact::new();
                match &mut selected_user.contact {
                    None => {
                        selected_user.contact =
                            Some(HashMap::from([(new_contact.email.clone(), new_contact)]));
                    }
                    Some(contacts) => {
                        contacts.insert(new_contact.email.clone(), new_contact);
                    }
                }
                MenuOption::UserMainMenu
            }
            MenuOption::Quit => {
                println!("Good bye!",);
                exit(0)
            }
            opt => not_implelemted_yet(format!("{:?}", opt).as_str()),
        };
    }
}

fn not_implelemted_yet(section: &str) -> MenuOption {
    println!("{} - Not implemented yet", section);
    MenuOption::Quit
}
