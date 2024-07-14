// ! TODO: Need to fix those strange duplicated imports (review modules)
use contact_book::{
    cb_io::cb_io::{read_data, save_data},
    cb_menu::cb_menu::create_menu,
    cb_user::cb_user::User,
};
fn main() {
    loop {
        let selection = create_menu();
        match selection {
            0 => {
                println!("Selected to register a user");
                let u = User::new();
                let users = vec![u];
                println!("Saving data...");
                save_data(&users, "data.json")
            }
            1 => {
                println!("Selected to register a contact");
                // ! for now it will save directly on the only user saved
                let mut users = read_data("data.json");
                let u = &mut users[0];
                u.add_contact();
                println!("Saving data...");
                save_data(&users, "data.json")
            }
            2 => {
                println!("Selected list contacts from a user");
                // ! for now it will show the content of the file
                let users = read_data("data.json");
                println!("{users:#?}");
            }
            3 => {
                println!("Selected to quit the app");
                std::process::exit(0);
            }
            _ => {
                unreachable!();
            }
        }
    }
}
