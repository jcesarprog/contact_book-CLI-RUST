mod app;
mod io;
mod menu;
mod user;

use std::process::exit;

use app::AppState;
use menu::MenuState;
use user::User;

fn main() {
    let mut app = AppState::new();
    let mut users = io::load_data();

    let mut menu_state = MenuState::MainMenu;
    loop {
        menu_state = match menu_state {
            MenuState::MainMenu => menu::main_menu(&app),
            MenuState::UserMenu => menu::user_menu(&app),
            MenuState::UserSelectionMenu => {
                // Limit the scope of the mutable borrow of `users` and `app`
                let next_state = menu::user_selection_menu(&mut users, &mut app);
                next_state
            }
            MenuState::Quit => {
                println!("Good Bye!");
                exit(0);
            }
            _ => unreachable!(),
        };
    }
}
