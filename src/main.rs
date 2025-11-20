#[allow(unused)]
mod modules;
use modules::colors::*;
use modules::path::*;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    loop {
        // showing current dir in prompt
        let cwd = env::current_dir().unwrap_or_else(|_| "/".into());
        let cwd_display = tilde(cwd.clone());
        // Get the username from the environment variable
        let username = env::var("USER").unwrap_or_else(|_| String::from("unknown_user"));
        // get hostname
        let hostname = modules::syshost::get_host();
        print!(
            "[{}{}{}@{}{}{}:{}{}{}]$ ",
            // ------------------
            GREEN,
            username,
            RESET,
            // ------------------
            RED,
            hostname,
            RESET,
            // ------------------
            CYAN,
            // cwd.display(),
            cwd_display,
            RESET
        );
        io::stdout().flush().unwrap();

        // read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!();
            break;
        }

        // it continues when the arguments is empty
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // exit build-in
        if input == "exit" {
            println!("kalesh: exiting...");
            break;
        }

        // parse tokens
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        // // handle build-in cd
        // if parts[0] == "cd" {
        //     let target = parts.get(1).cloned().unwrap_or("/");
        //     if let Err(err) = env::set_current_dir(Path::new(target)) {
        //         eprintln!("kalesh: cd: {}: {}", target, err);
        //     }
        //     continue;
        // }

        if parts[0] == "cd" {
            // handle: `cd` →  go to home
            let target = if parts.len() == 1 {
                // $HOME environment variable
                env::var("HOME").unwrap_or_else(|_| "/".to_string())
            } else {
                parts[1].to_string()
            };

            if let Err(err) = env::set_current_dir(Path::new(&target)) {
                eprintln!("kalesh: cd: {}: {}", target, err);
            }
            continue;
        }

        // external commands
        let mut cmd = Command::new(parts[0]);
        cmd.args(&parts[1..]);

        // inherit stdin/stdout/stderr
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        match cmd.spawn() {
            Ok(mut child) => {
                child.wait().expect("failed to wait on child");
            }
            Err(err) => {
                eprintln!("kalesh: command not found: {}", err);
            }
        }
    }
}
