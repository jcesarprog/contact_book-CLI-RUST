use crate::user::User;

pub struct AppState<'a> {
    pub user_selected: Option<&'a mut User>,
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        Self {
            user_selected: None,
        }
    }

    pub fn get_user(&self) -> String {
        let res = match &self.user_selected {
            None => "No user selected".to_string(),
            Some(user) => format!("{}", user.name),
        };
        // println!("{}", res);
        res
    }

    pub fn set_user(&mut self, user: &'a mut User) {
        self.user_selected = Some(user);
    }

    pub fn clear_state(&mut self) {
        self.user_selected = None;
    }
}
