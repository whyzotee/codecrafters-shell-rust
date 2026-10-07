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

        let all_cmd = vec!["exit", "echo", "type"];

        if let Some(cmd) = command.split(" ").nth(0) {
            match cmd {
                "exit" => break,
                "echo" => {
                    let words: Vec<&str> = command.split(" ").skip(1).collect();
                    println!("{}", words.join(" "))
                }
                "type" => {
                    let args: Vec<&str> = command.split(" ").skip(1).collect();

                    if args.len() == 1 && all_cmd.contains(&args[0]) {
                        println!("{} is a shell builtin", &args[0]);
                    } else {
                        println!("{}: not found", &args[0]);
                    }
                }
                _ => println!("{}: command not found", command),
            }
        }
    }
}
