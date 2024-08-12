use std::{collections::HashMap, io::Error};

use crate::{contact::Contact, user::User};

pub fn load_users() -> Result<HashMap<String, User>, Error> {
    let u1 = User {
        name: "user 1".to_string(),
        email: "user1@mail".to_string(),
        contact: None,
    };
    let u2 = User {
        name: "user 2".to_string(),
        email: "user2@mail".to_string(),
        contact: None,
    };
    let u3 = User {
        name: "user 3".to_string(),
        email: "user3@mail".to_string(),
        contact: Some(vec![
            Contact {
                name: "contact1".to_string(),
                email: "contact1@mail".to_string(),
                phone: None,
            },
            Contact {
                name: "contact2".to_string(),
                email: "contact2@mail".to_string(),
                phone: Some("123-123-123".to_string()),
            },
        ]),
    };
    let mut users = HashMap::new();
    users.insert(u1.email.clone(), u1);
    users.insert(u2.email.clone(), u2);
    users.insert(u3.email.clone(), u3);
    Ok(users)
}
