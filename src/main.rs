use std::io::SeekFrom::Start;
#[allow(unused_imports)]
//use std::env;
use std::io::{self, Write};
use std::process::Command;
//use std::os::unix::fs::PermissionsExt;
//use std::path::PathBuf;
//use std::{fs, path};
//use std::path::PathBuf;

/*
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        if command == "exit" {
            break;
        } else if command.starts_with("echo") {
            println!("{}", &command[5..]);
        } else if command.starts_with("type") {
            if &command[5..] == "echo" || &command[5..] == "type" || &command[5..] == "exit" {
                println!("{} is a shell builtin", &command[5..]);
            } else {
                let key = "PATH";
                match env::var_os(key) {
                    Some(paths) => {
                        for path in env::split_paths(&paths) {
                            //println!("'{}'", path.display());
                            //println!("paths: {}", paths.display());
                            for file in &path {
                                //println!("file {:?} in &path {:?}", file, &path);
                                if *file == command[5..]
                                    && path.metadata().unwrap().permissions().mode() & 0o100 != 0
                                {
                                    println!("{} is {}", command, path.display());
                                }
                            }
                        }
                    }
                    None => println!("None"),
                }
            }
        }
    }
}
*/

fn main() {
    let available_commands = ["type", "echo", "exit"];

    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();
        let mut input_parts = input.splitn(2, ' ');

        let command = input_parts.next().unwrap_or("");
        let args = input_parts.next().unwrap_or("");

        if command == "exit" {
            break;
        } else if command == "echo" {
            println!("{}", args);
        } else if command == "type" {
            if available_commands.contains(&args) {
                println!("{} is a shell builtin", args);
            } else if let Ok(path) = which::which(args) {
                println!("{} is {}", args, path.display());
                // TODO: 1. Determine if the given command is an executable (you can reuse the logic from type
                // TODO: 2. If it is, execute the program
                // TODO: 3. Pass any arguments from the command line to the program
                // For example, if the user types custom_exe arg1 arg2, your shell should:

                /* Determine if custom_exe is an executable in PATH
                    Execute it with three arguments: custom_exe (the program name), arg1, and arg2
                */
            } else {
                println!("{}: not found", args);
            }
        } else if let Ok(path) = which::which(args) {
            let program = Command::new(command);
            println!("program: {:?}", &program);
        } else {
            println!("{}: command not found", command);
        }
    }
}
