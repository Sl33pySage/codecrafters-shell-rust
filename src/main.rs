use std::env;
use std::ffi::OsStr;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;

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
            // TODO: 2. If the command is not a builtin, your shell must go through every directory in PATH. For each directory:
            //  TODO: a. Check if a file with the command name exists.
            //  TODO: b. Check if the file has execute permissions.
            //  TODO: c. If the file exists and has execute permissions, print <command> is
            //  <full_path> and stop.
            //  TODO: d. If the file exists but lacks execute permissions, skip it and continue to the next directory.
            // TODO: 3. If no executable is found in any directory, print <command> : not found.
            if &command[5..] == "echo" || &command[5..] == "type" || &command[5..] == "exit" {
                println!("{} is a shell builtin", &command[5..]);
            } else {
                let key = "PATH";
                match env::var_os(key) {
                    Some(paths) => {
                        for path in env::split_paths(&paths) {
                            println!("'{}'", path.display());
                            for file in &path {
                                println!("{:?}", &file);
                            }
                        }
                    }
                    None => println!("{key} is not defined in the enviroment."),
                }
                println!("{}: not found", &command[5..]);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
