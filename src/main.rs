use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, Write};

fn main() {
    print!("Log in (l) or sign up (s): ");

    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    let choice = choice.trim().to_lowercase();
    match choice.as_str() {
        "l" => {
            log_in();
        },
        "s" => {
            sign_up();
        },
        _ => {
            println!("Character not recognized.")
        }
    }
}

fn log_in() {
    println!("log in");
    print!("enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    
    print!("enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    
    let username = username.trim();
    let password = password.trim();
    
    println!("\nusername: {}\npassword: {}", username, password);
    
    let combination = format!("{},{}", username, password);

    let file_content = read("credentials.txt");

    if file_content.lines().any(|line| line.trim() == combination) {
        println!("successfully logged in");
    } else {
        println!("invalid username or password");
    }
}

fn sign_up() {
    println!("sign up");
    print!("enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    
    print!("enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    
    let username = username.trim();
    let password = password.trim();
    
    println!("\nusername: {}\npassword: {}", username, password);
    
    let combination = format!("{},{}\n", username, password);
    write("credentials.txt", &combination);
}

fn read(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => s,
        Err(why) => panic!("couldn't read {}: {}", display, why),
    }
}

fn write(filename: &str, content: &str) {
    let mut file_content = read(filename);

    if file_content.lines().any(|line| line.split(',').next() == Some(content.split(',').next().unwrap())) {
        println!("username already exists");
        return;
    }

    file_content.push_str(content);
    
    let path = Path::new(filename);
    let display = path.display();
    
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path) 
    {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", display, why),
    };
    
    match file.write_all(file_content.as_bytes()) {
        Ok(_) => println!("successfully wrote to {}", display),
        Err(why) => panic!("couldn't write to {}: {}", display, why),
    }
}