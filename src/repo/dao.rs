use std::{collections::HashMap, io::Error};

use super::models::{contact::Contact, user::User};

pub trait DAO {
    fn get_users(&self) -> Result<HashMap<String, User>, Error>;
    fn save_user(&self, users: &HashMap<String, User>) -> Result<(), Error>;
    fn remove_user(
        &self,
        key: &str,
        users: HashMap<String, User>,
    ) -> Result<HashMap<String, User>, Error>;

    // fn get_user(&self, key: &str) -> Result<Option<User>, Error>;
    // fn update_user(&self, key: &str, user: User) -> Result<HashMap<String, User>, Error>;

    // fn get_contacts(user: User) -> Result<HashMap<String, Contact>, Error>;
    // fn get_contact(key: String) -> Result<Contact, Error>;
    // fn create_contact(contact: Contact) -> Result<Contact, Error>;
    // fn update_contact(key: String, contact: Contact) -> Result<Contact, Error>;
    // fn remove_contact(key: String) -> Result<(), Error>;
}
