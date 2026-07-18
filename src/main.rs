#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        // Your Program should:
        // 1. ✅ Display the prompt $
        print!("$ ");
        io::stdout().flush().unwrap();

        // 2. Read the user's input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        } else {
            // 3. Print an error message in exactly this format: {command}: command not found
            //  ๏ e.g. if the user types xyz, print xyz: command not found
            println!("{}: command not found", input.trim());
        }
    }
}
