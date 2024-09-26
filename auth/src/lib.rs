pub fn greet(name: &str) -> String {
    format!("Hello {name}")
}

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
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
    fn test_greet() {
        let result = greet("Ash");
        assert_eq!(result, "Hello Ash");
    }

    #[test]
    fn test_login() {
        assert!(login("admin", "password"));
        assert!(login("ADMIN", "password"));
        assert!(!login("ash", "password"));
        assert!(!login("ash", "mypass"));
    }
}
