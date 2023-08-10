use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserRow {
    pub username: String,
    pub password: String,
}


pub type UserConfig = HashMap<String, UserRow>;


pub struct UserConfigParser;

impl UserConfigParser {
    pub fn user_config(value: &str) -> UserConfig {
        let mut users = HashMap::new();

        let collects: Vec<_> = value
            .split(",")
            .collect();

        for i in 0..collects.len() {
            let row = collects[i];
            let user: Vec<_> = row.split(":").collect();
            if user.len() == 2 {
                let username = user.first().unwrap().to_string();
                let password = user.last().unwrap().to_string();
                let username_clone = username.clone();
                users.insert(username, UserRow {
                    username: username_clone,
                    password,
                });
            }
        }
        users
    }
}
