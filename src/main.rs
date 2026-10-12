use std::io::{self, Write};
use std::{env, fs};

fn find_command_in_path(cmd: &str, path: &str) -> Option<String> {
    if let Ok(paths) = env::var(path) {
        for path in env::split_paths(&paths) {
            let files = fs::read_dir(path);

            if files.is_err() {
                continue;
            }

            let files = files.unwrap();

            for f in files {
                if f.is_err() {
                    continue;
                }

                let f = f.unwrap().path().display().to_string();

                if f.split("/").last().unwrap() == cmd {
                    return Some(f);
                }
            }
        }
    }

    None
}

fn handle_command(cmd: &str, args: Vec<&str>, path_key: &str) -> Option<i32> {
    let all_cmd = vec!["exit", "echo", "type"];

    if cmd == "exit" {
        return Some(-1);
    } else if cmd == "echo" {
        println!("{}", args.join(" "));
    } else if cmd == "type" {
        if args.len() == 1 && all_cmd.contains(&args[0]) {
            println!("{} is a shell builtin", &args[0]);
        } else {
            if let Some(path) = find_command_in_path(&args[0], path_key) {
                println!("{} is {}", &args[0], path)
            } else {
                println!("{}: not found", &args[0]);
            }
        }
    } else {
        return Some(1);
    }

    None
}

fn main() {
    let key = "PATH";

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command = command.trim();

        let args: Vec<&str> = command.split(" ").skip(1).collect();

        if let Some(cmd) = command.split(" ").nth(0) {
            if let Some(return_code) = handle_command(cmd, args, key) {
                if return_code == -1 {
                    break;
                }

                if return_code == 1 {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
