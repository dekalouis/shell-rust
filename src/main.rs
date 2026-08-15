#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .unwrap();
        
        let command = command.trim().to_string();

        if command == "exit" {
            break;
        }  
        if command.starts_with("echo ") {
            println!("{}", &command[5..]);
            continue;
        }
        if command.starts_with("type ") {
            let arg = &command[5..];
            if matches!(arg, "echo" | "exit" | "type") {
                println!("{arg} is a shell builtin");
            } else if let Ok(path) = which::which(&command[5..]) {
                println!("{} is {}", &command[5..], path.display());
            } else {
                println!("{}: not found", arg);
            }
            continue;
        }

        println!("{}: command not found", command.trim());
    }
}
