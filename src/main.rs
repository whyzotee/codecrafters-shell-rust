use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};

fn find_command_in_path(cmd: &str, path: &str) -> Option<String> {
    let all_path: Vec<&str> = path.split("/").collect();

    for p in all_path {
        if let Some(paths) = env::var_os(p) {
            for path in env::split_paths(&paths) {
                if path.display().to_string().contains(cmd) {
                    return Some(path.display().to_string());
                }
            }
        }
    }

    None
}

fn handle_command(cmd: &str, args: Vec<&str>, path_key: &str) -> i32 {
    let all_cmd = vec!["exit", "echo", "type"];

    match cmd {
        "exit" => -1,
        "echo" => {
            println!("{}", args.join(" "));

            0
        }
        "type" => {
            if args.len() == 1 && all_cmd.contains(&args[0]) {
                println!("{} is a shell builtin", &args[0]);
            } else {
                if let Some(path) = find_command_in_path(&args[0], path_key) {
                    println!("{} is {}", &args[0], path)
                } else {
                    println!("{}: not found", &args[0]);
                }
            }

            0
        }
        _ => 1,
    }
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
            let return_code = handle_command(cmd, args, key);

            if return_code == 1 {
                println!("{}: command not found", command);
            }

            if return_code == -1 {
                break;
            }
        }
    }
}
