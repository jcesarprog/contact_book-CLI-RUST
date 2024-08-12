use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug)]
pub enum MenuOption {
    MainMenu,          // Menu with select and register user
    ListUsersToSelect, // Menu with the list of users to select
    UserMainMenu,      // Menu with the options for a selected user
    Quit,
    RegisterUser,
}

pub fn menu_select_register_user() -> MenuOption {
    let selections = &["Select a user", "Register a user"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => MenuOption::ListUsersToSelect,
        1 => MenuOption::RegisterUser,
        _ => unreachable!(),
    }
}
