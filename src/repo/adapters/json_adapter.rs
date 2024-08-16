use crate::repo::dao::DAO;
use crate::repo::models::user::User;
use std::collections::HashMap;
use std::io::Error;
pub struct UserJsonAdapter {
    pub file_path: String,
}

impl DAO for UserJsonAdapter {
    // TODO
}
