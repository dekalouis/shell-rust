#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;

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
            // if &command[5..] == "echo" || &command[5..] == "exit" {
            if matches!(&command[5..], "echo" | "exit" | "type") {
                println!("{} is a shell builtin", &command[5..]);
            } else {
                // println!("{}: not found", &command[5..]);
                let cmd = &command[5..];
                let path_var = env::var("PATH").unwrap_or_default();
                // println!("{path_var}");
                let found = path_var
                    .split(':')
                    .map(|dir| Path::new(dir).join(cmd))
                    .find(|full_path| full_path.is_file());
                
                match found {
                    Some(full_path) => println!("{cmd} is {}", full_path.display()),
                    None => println!("{cmd}: not found"),
                }
            }
            continue;
        }

        println!("{}: command not found", command.trim());
    }
}
