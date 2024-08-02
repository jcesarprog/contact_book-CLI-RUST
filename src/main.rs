mod app;
mod io;
mod menu;
mod user;

use app::AppState;
use user::User;

fn main() {
    let mut app = AppState::new();
    let mut users = io::load_data();

    match menu::create_main_menu(&app) {
        0 => {
            // Select User

            // * SELECTING A USER
            let selected_idx = menu::create_generic_menu(
                "Choose an user",
                &extract_names_from_users(&users, true),
            );
            if selected_idx == users.len() {
                // Back chosen
                println!("Back chosen");
                return;
            }
            app.set_user(&mut users[selected_idx]);

            // * USER MENU
            let (selected_idx, back_idx) = menu::create_user_menu(&app);

            if selected_idx == back_idx {
                // Back chosen
                println!("Back chosen");
                return;
            }
            match selected_idx {
                0 => {
                    println!("List contacts")
                }
                1 => {
                    println!("Edit User")
                }
                _ => unreachable!(),
            }
        }
        1 => {
            // Register an user
            println!("Selected 1")
        }
        2 => {
            println!("Good Bye!")
        }
        _ => unreachable!(),
    }
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
