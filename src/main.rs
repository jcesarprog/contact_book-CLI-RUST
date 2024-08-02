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

    let mut menu_state = match menu::main_menu(&app) {
        MenuState::UserSelectionMenu => menu::user_selection_menu(&mut users, &mut app),
        // ! Need to implement this register_user function
        MenuState::RegisterUser => register_user(&mut users, &mut app),
        _ => {
            println!("Good bye");
            exit(0)
        }
    };
    {
        match menu_state {
            MenuState::UserMenu => menu::user_menu(&app),
            MenuState::MainMenu => menu::main_menu(&app),
            _ => unreachable!(),
        };
    }

    // while menu_state != MenuState::Quit {
    //     menu_state = match menu_state {
    //         MenuState::MainMenu => menu::main_menu(&app),
    //         MenuState::UserMenu => menu::user_menu(&app),
    //         MenuState::UserSelectionMenu => menu::user_selection_menu(&mut users, &mut app),
    //         _ => MenuState::Quit,
    //     };
    // }
    // ###############
    // match menu::create_main_menu(&app) {
    //     0 => {
    //         // Select User

    //         // * SELECTING A USER
    //         let selected_idx = menu::create_generic_menu(
    //             "Choose an user",
    //             &extract_names_from_users(&users, true),
    //         );
    //         if selected_idx == users.len() {
    //             // Back chosen
    //             println!("Back chosen");
    //             return;
    //         }
    //         app.set_user(&mut users[selected_idx]);

    //         // * USER MENU
    //         let (selected_idx, back_idx) = menu::create_user_menu(&app);

    //         if selected_idx == back_idx {
    //             // Back chosen
    //             println!("Back chosen");
    //             return;
    //         }
    //         match selected_idx {
    //             0 => {
    //                 println!("List contacts")
    //             }
    //             1 => {
    //                 println!("Edit User")
    //             }
    //             _ => unreachable!(),
    //         }
    //     }
    //     1 => {
    //         // Register an user
    //         println!("Selected 1")
    //     }
    //     2 => {
    //         println!("Good Bye!")
    //     }
    //     _ => unreachable!(),
    // }
}

fn register_user(users: &mut Vec<User>, app: &mut AppState) -> MenuState {
    MenuState::UserMenu
}

fn extract_names_from_users(users: &Vec<User>, put_back: bool) -> Vec<&str> {
    let mut names: Vec<&str> = Vec::new();
    for u in users {
        names.push(&u.name);
    }

    if put_back == true {
        names.push("<- Back");
    }
    names
}

fn selecting_a_user<'a>(app: &'a mut AppState<'a>, users: &'a mut Vec<User>) {
    // * SELECTING A USER
    let selected_idx =
        menu::create_generic_menu("Choose an user", &extract_names_from_users(&users, true));
    if selected_idx == users.len() {
        // Back chosen
        println!("Back chosen");
        return;
    }
    app.set_user(&mut users[selected_idx]);
}
