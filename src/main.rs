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
                utils::clear_terminal_and_show_user(&app);
                menu::menu_select_register_user()
            }
            MenuOption::RegisterUser => {
                let u = User::new();
                users.insert(u.email.clone(), u);
                println!("{:#?}", users);
                MenuOption::UserMainMenu
            }
            MenuOption::ListUsersToSelect => menu::menu_list_users_to_select(&mut app, &users),

            MenuOption::UserMainMenu => not_implelemted_yet("User Main Menu"),
            MenuOption::Quit => {
                println!("Good bye!",);
                exit(0)
            }
            _ => unreachable!(),
        };
    }
}

fn not_implelemted_yet(section: &str) -> MenuOption {
    println!("{} - Not implemented yet", section);
    MenuOption::Quit
}
