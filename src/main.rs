#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .unwrap();
        
        let command = command.trim().to_string();
        // path var 
        let path_var = env::var("PATH").unwrap_or_default();

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
            } else if let Some(matched_path) = path_var
                .split(':')
                .map(|dir| Path::new(dir).join(arg))
                .find(|file_path| {
                    file_path.is_file() && file_path
                        .metadata()
                        .map(|meta| meta.permissions().mode() & 0o111 != 0)
                        .unwrap_or(false)
                })
            {
                println!("{arg} is {}", matched_path.display());
            // } else if let Ok(path) = which::which(&command[5..]) {
            //     println!("{} is {}", &command[5..], path.display());
            } else {
                println!("{}: not found", arg);
            }
            continue;
        }

        println!("{}: command not found", command.trim());
    }
}
