use auth::*;

fn main() {
    let mut tries: i32 = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Admin"),
                    LoginRole::User => println!("User"),
                }
                break;
            }
            Some(LoginAction::Denied) => {
                println!("Incorrect password, access denied")
            }
            None => {
                println!("User does not exist")
            }
        }

        tries += 1;
        if tries >= 3 {
            println!("Too many failed attempts");
            break
        }
    }
}
