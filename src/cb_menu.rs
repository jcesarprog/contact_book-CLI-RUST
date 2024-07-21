use dialoguer::{theme::ColorfulTheme, Select};
pub fn create_menu() -> usize {
    let menu_items = [
        "Register a user",
        "Select a user",
        "Register a contact into a user",
        "List contacts from a user",
        "Quit",
    ];
    println!("\nWelcome to contact book CLI app!");

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .items(&menu_items)
        .default(0)
        .interact()
        .unwrap();

    return selection;
}
