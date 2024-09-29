use auth::{get_users, hash_password, save_users, LoginRole, User};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users
    List,
    /// Add new user
    Add {
        /// Username of new user
        username: String,

        /// Password of new user (plaintext)
        password: String,

        /// Optional - set new user to be admin
        admin: Option<bool>
    },
    /// Delete user
    Delete {
        /// Username of user to delete
        username: String
    },
    /// Change the password of an existing user
    ChangePassword {
        /// Username of user that will have the changed password
        username: String,
        /// The new password
        password: String
    }
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = get_users();
    users
        .iter()
        .for_each(|user| {
            println!("{:<20}{:<20?}", user.username, user.role)
        });

}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    let new_user = User::new(&username, &password, role);
    users.push(new_user);
    save_users(users)
}

fn delete_user(username: String) {
    let mut users = get_users();
    let user_index = users.iter().position(|user| user.username == username);
    match user_index {
        Some(i) => {
            users.remove(i);
            save_users(users);
        }
        None => {println!("{username} does not exist")}
    }
}

fn change_password(username: String, password: String) {
    let mut users = get_users();
    let user_index = users.iter().position(|user| user.username == username);
    match user_index {
        Some(i) => {
            users.get_mut(i).unwrap().password = hash_password(&password);
            save_users(users);
        }
        None => {println!("{username} does not exist")}
    }
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users()
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword { username, password }) => {
            change_password(username, password);
        }
        None => {
            print!("Use --help to see list of commands")
        }
    }
}
