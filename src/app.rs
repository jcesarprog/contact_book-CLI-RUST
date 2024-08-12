use super::menu::MenuOption;
#[derive(Debug)]
pub struct AppState {
    pub menu_state: MenuOption,
    pub user_selected: Option<String>,
    pub contact_selected: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            menu_state: MenuOption::MainMenu,
            user_selected: None,
            contact_selected: None,
        }
    }
}
