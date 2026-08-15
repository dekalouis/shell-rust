#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

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
        
        let valid_args = ["echo", "exit", "type", "pwd", "cd"];

        if command == "exit" {
            break;
        } else if command.starts_with("echo ") {
            println!("{}", &command[5..]);
            continue;
        } else if command == "pwd" {
            if let Ok(path) = env::current_dir() {
                println!("{}", path.display());
            }
            continue;
        } else if command.starts_with("type ") {
            // running type with a matched_path
            let arg = &command[5..];
            // if matches!(arg, "echo" | "exit" | "type") {
            if valid_args.contains(&arg) {
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
        } else if command.starts_with("cd ") {
            let path = &command[3..];
            // println!("printed arg {path}");
            if let Err(_) = std::env::set_current_dir(path) {
                println!("cd: {path}: No such file or directory");
            }
            continue;
        } else {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if let Some(&program) = parts.first() {
                let args = &parts[1..];
                match Command::new(program).args(args).status() {
                    Ok(_) => {}
                    Err(_) => println!("{}: command not found", command.trim()),
                }
                continue;
            }
        }
    }
}
