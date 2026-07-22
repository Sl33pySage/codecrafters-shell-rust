#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn plswork(path: PathBuf, command: String) -> std::io::Result<()> {
    let metadata = fs::metadata(&path)?;
    // Retrieve the raw bits (u32)
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    // Print the octal format (Standard unix representation like 755)
    //println!("{:o}", mode);
    // Check if the lower 9 bits contain specefic permissions (e.g., owner execute)
    let is_owner_executable = (mode & 0o100) != 0;
    if path.exists() && is_owner_executable {
        //  TODO: c. If the file exists and has execute permissions, print <command> is
        //  <full_path> and stop.
        println!("{} is {}", command, path.display());
    }
    if path.exists() && !is_owner_executable {
        //  TODO: d. If the file exists but lacks execute permissions, skip it and continue to the next directory.
        println!("False: {}", is_owner_executable);
    } else {
        // TODO: 3. If no executable is found in any directory, print <command> : not found.

        println!("{}: command not found", command);
    }
    Ok(())
}

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
                // TODO: 2. If the command is not a builtin, your shell must go through every directory in PATH. For each directory:
                let key = "PATH";
                match env::var_os(key) {
                    Some(paths) => {
                        for path in env::split_paths(&paths) {
                            //println!("'{}'", path.display());
                            for file in &path {
                                //  TODO: a. Check if a file with the command name exists.
                                if file == &command[5..] {
                                    //  TODO: b. Check if the file has execute permissions.
                                    plswork(path.clone(), command[5..].to_string());
                                }
                            }
                        }
                    }
                    None => println!("{key} is not defined in the enviroment."),
                }
                println!("{}: not found", &command[5..]);
            }
        }
    }
}
