use std::{collections::HashMap, io::Error};

use super::models::{contact::Contact, user::User};

pub trait DAO {
    fn get_users() -> Result<HashMap<String, User>, Error>;
    fn get_user(key: String) -> Result<User, Error>;
    fn create_user(user: User) -> Result<User, Error>;
    fn update_user(key: String, user: User) -> Result<User, Error>;
    fn remove_user(key: String) -> Result<(), Error>;

    fn get_contacts(user: User) -> Result<HashMap<String, Contact>, Error>;
    fn get_contact(key: String) -> Result<Contact, Error>;
    fn create_contact(contact: Contact) -> Result<Contact, Error>;
    fn update_contact(key: String, contact: Contact) -> Result<Contact, Error>;
    fn remove_contact(key: String) -> Result<(), Error>;
}
