#[derive(Debug)]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub contacts: Option<Vec<Contact>>,
}
