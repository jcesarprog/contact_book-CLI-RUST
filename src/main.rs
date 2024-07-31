mod app;
mod io;
mod menu;
mod user;

use app::AppState;

fn main() {
    let mut app = AppState::new();

    let mut users = io::load_data();
    println!("Users: {:#?}", &users);
    app.get_user();
    // app.set_user(&mut users[0]);
    app.get_user();

    match menu::create_main_menu(&app) {
        0 => println!("Selected 0"),
        1 => println!("Selected 1"),
        val => println!("Selected something else: {val}"),
    }
}
