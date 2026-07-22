#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn plswork(path: PathBuf) -> std::io::Result<()> {
    let metadata = fs::metadata(&path)?;
    println!("{:o}", !metadata.permissions().mode());
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
            //  TODO: c. If the file exists and has execute permissions, print <command> is
            //  <full_path> and stop.
            //  TODO: d. If the file exists but lacks execute permissions, skip it and continue to the next directory.
            // TODO: 3. If no executable is found in any directory, print <command> : not found.
            if &command[5..] == "echo" || &command[5..] == "type" || &command[5..] == "exit" {
                println!("{} is a shell builtin", &command[5..]);
            } else {
                // TODO: 2. If the command is not a builtin, your shell must go through every directory in PATH. For each directory:
                let key = "PATH";
                match env::var_os(key) {
                    Some(paths) => {
                        for path in env::split_paths(&paths) {
                            println!("'{}'", path.display());
                            for file in &path {
                                //  TODO: a. Check if a file with the command name exists.
                                if file == &command[5..] {
                                    //checking for permissions
                                    // println!("MATCH: {:?}", file);
                                    //  TODO: b. Check if the file has execute permissions.

                                    //let permissions = metadata.permissions();
                                    //println!("{:?}", permissions);
                                    plswork(path.clone());
                                }
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
