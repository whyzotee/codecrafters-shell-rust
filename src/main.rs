#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command = command.trim();

        if let Some(cmd) = command.split(" ").nth(0) {
            match cmd {
                "exit" => break,
                "echo" => {
                    let words: Vec<&str> = command.split(" ").skip(1).collect();
                    println!("{}", words.join(" "))
                }
                _ => println!("{}: command not found", command),
            }
        }
    }
}
