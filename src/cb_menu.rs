pub mod cb_menu {
    use dialoguer::{theme::ColorfulTheme, Select};
    pub fn create_menu() -> usize {
        let menu_items = [
            "1: Register a user",
            "2: Register a contact into a user",
            "3: List contacts from a user",
            "4: Quit",
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
}
