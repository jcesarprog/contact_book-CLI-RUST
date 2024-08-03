mod app;
mod io;
mod menu;
mod user;

use app::AppState;
use menu::MenuState;

fn main() {
    let mut app = AppState::new();
    let mut users = io::load_data();

    let mut menu_state = MenuState::MainMenu;
    loop {
        menu_state = match menu_state {
            MenuState::MainMenu => menu::main_menu(&app),
            MenuState::UserMenu => menu::user_menu(&app),
            MenuState::UserSelectionMenu => {
                // ! Need to Limit the scope of the mutable borrow of `users` and `app`
                let next_state = menu::user_selection_menu(&mut users, &mut app);
                next_state
            }
            MenuState::Quit => {
                println!("Good Bye!");
                std::process::exit(0);
            }
            _ => unreachable!(),
        };
    }
}
