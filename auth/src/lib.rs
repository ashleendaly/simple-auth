#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: password.to_string(),
            role
        }
    }
}

pub fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("ash", "password", LoginRole::User),
    ]
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
        if user.password == password {
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
