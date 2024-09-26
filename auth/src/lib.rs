#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if username != "admin" && username != "ash" {
        return None
    }
    
    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "ash" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
}

pub fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_login() {
    //     assert_eq!(login("admin", "password"), LoginAction::Granted(LoginRole::Admin));
    //     assert_eq!(login("ADMIN", "password"), LoginAction::Granted(LoginRole::Admin));
    //     assert_eq!(login("ash", "password"), LoginAction::Granted(LoginRole::User));
    //     assert_eq!(login("ash", "not-password"), LoginAction::Denied);
    // }
}
