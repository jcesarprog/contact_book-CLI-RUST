#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}
