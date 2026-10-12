use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let key = "PATH";

    'outer: loop {
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
                        match env::var_os(key) {
                            Some(paths) => {
                                for path in env::split_paths(&paths) {
                                    if path.display().to_string().contains(&args[0]) {
                                        println!("{} is {}", &args[0], path.display());
                                        continue 'outer;
                                    }
                                }
                            }
                            None => println!("{key} is not defined in the environment."),
                        }

                        println!("{}: not found", &args[0]);
                    }
                }
                _ => println!("{}: command not found", command),
            }
        }
    }
}
