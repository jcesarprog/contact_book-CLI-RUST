use super::menu::MenuOptionAndAction;
#[derive(Debug)]
pub struct AppState {
    pub menu_state: MenuOptionAndAction,
    pub user_selected: Option<String>,
    pub contact_selected: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            menu_state: MenuOptionAndAction::MainMenu,
            user_selected: None,
            contact_selected: None,
        }
    }
}
