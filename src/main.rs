mod app;
mod contact;
mod io;
mod menu;
mod user;
mod utils;

use app::AppState;
use dialoguer::{theme::ColorfulTheme, Input};
use menu::MenuOption;
use user::User;
use utils::show_user;

fn main() {
    let mut app = AppState::new();
    let mut users = io::load_users().expect("Error loading users");

    loop {
        app.menu_state = match app.menu_state {
            MenuOption::MainMenu => {
                show_user(&app, &users);
                menu::menu_select_register_user()
            }
            MenuOption::RegisterUser => {
                let u = User::new();
                users.push(u);
                println!("{:#?}", users);
                MenuOption::UserMainMenu
            }
            _ => MenuOption::Quit,
        };
    }
}
