mod app;
mod menu;
mod repo;
mod utils;

use app::AppState;
use menu::MenuOptionAndAction;
use repo::{adapters::json_adapter::UserJsonAdapter, dao::DAO};

fn main() {
    let adapter_dao = UserJsonAdapter {
        file_path: String::from("data.json"),
    };
    let mut app = AppState::new();

    let mut users = adapter_dao.get_users().expect("Error loading users");

    loop {
        app.menu_state = match app.menu_state {
            MenuOptionAndAction::MainMenu => {
                utils::clear_terminal_and_show_user(&app, &users);
                menu::menu_select_register_user()
            }
            MenuOptionAndAction::RegisterUser => {
                menu::menu_register_user(&mut app, &mut users, &adapter_dao)
            }
            MenuOptionAndAction::ListUsersToSelect => {
                menu::menu_list_users_to_select(&mut app, &users)
            }

            MenuOptionAndAction::UserMainMenu => menu::menu_user_menu(&app, &users),
            MenuOptionAndAction::EditUser => menu::menu_edit_user(&mut app, &mut users),
            MenuOptionAndAction::AddContact => menu::menu_add_contact(&mut app, &mut users),
            MenuOptionAndAction::ListContacts => {
                menu::menu_list_contacts_to_select(&mut app, &users)
            }
            MenuOptionAndAction::EditContact => menu::menu_edit_contact(&mut app, &mut users),
            MenuOptionAndAction::RemoveUser => {
                println!("Removing user...");
                users = menu::menu_remove_user(&mut app, users, &adapter_dao);
                MenuOptionAndAction::MainMenu
            }
            MenuOptionAndAction::Quit => {
                println!("Good bye!",);
                std::process::exit(0)
            }
            other => utils::not_implelemted_yet(&format!("{:?}", other)),
        };
    }
}
