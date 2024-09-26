use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role
        }
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::{Sha256, Digest};

    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn get_default_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("ash", "password", LoginRole::User),
    ]
}

pub fn get_users() -> Vec<User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        serde_json::from_str(&users_json).unwrap()
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

pub fn get_admin_usernames() -> Vec<String> {
    get_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect()
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    if let Some(user) = users.iter().find(|user| user.username == username.to_lowercase()) {
        if user.password == hash_password(password) {
            return Some(LoginAction::Granted(user.role.clone()))
        } else {
            return Some(LoginAction::Denied)
        }
    }
    None
}

pub fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_admin_usernames() {
        assert_eq!(get_admin_usernames(), vec!["admin"])
    }
}
