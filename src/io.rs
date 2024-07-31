use crate::user::{Contact, User};

pub fn load_data() -> Vec<User> {
    vec![
        User {
            id: 0,
            name: "First".to_string(),
            email: "123@123.com".to_string(),
            contacts: None,
        },
        User {
            id: 1,
            name: "2nd".to_string(),
            email: "321@321.com".to_string(),
            contacts: Some(vec![
                Contact {
                    id: 11,
                    email: "11@11.com".to_string(),
                    name: "c01".to_string(),
                },
                Contact {
                    id: 12,
                    email: "12@12.com".to_string(),
                    name: "c02".to_string(),
                },
            ]),
        },
    ]
}
