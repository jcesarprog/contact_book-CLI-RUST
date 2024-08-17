mod app;
mod menu;
mod repo;
mod utils;

use app::AppState;
use menu::MenuOption;
use repo::{adapters::json_adapter::UserJsonAdapter, dao::DAO};

fn main() {
    let adapter = UserJsonAdapter {
        file_path: String::from("data.json"),
    };
    let mut app = AppState::new();

    let mut users = adapter.get_users().expect("Error loading users");

    loop {
        app.menu_state = match app.menu_state {
            MenuOption::MainMenu => {
                utils::clear_terminal_and_show_user(&app, &users);
                menu::menu_select_register_user()
            }
            MenuOption::RegisterUser => menu::menu_register_user(&mut app, &mut users, &adapter),
            MenuOption::ListUsersToSelect => menu::menu_list_users_to_select(&mut app, &users),

            MenuOption::UserMainMenu => menu::menu_user_menu(&app, &users),
            MenuOption::EditUser => menu::menu_edit_user(&mut app, &mut users),
            MenuOption::AddContact => menu::menu_add_contact(&mut app, &mut users),
            MenuOption::ListContacts => menu::menu_list_contacts_to_select(&mut app, &users),
            MenuOption::EditContact => menu::menu_edit_contact(&mut app, &mut users),
            MenuOption::Quit => {
                println!("Good bye!",);
                std::process::exit(0)
            }
        };
    }
}
