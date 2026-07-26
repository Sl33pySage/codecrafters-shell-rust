#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
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
