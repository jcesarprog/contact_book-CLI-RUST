mod app;
mod contact;
mod io;
mod menu;
mod user;
mod utils;

use std::process::exit;

use app::AppState;
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
