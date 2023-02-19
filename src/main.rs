use std::env;
use AeroKiTTY::term::{run, Term};

fn main() {
    // if cfg!(windows) {
    //     let shell = "C:\\Windows\\System32\\cmd.exe";
    //     run(shell.to_string());
    // } else if cfg!(unix) {
    //     let shell = env::var("SHELL").unwrap();
    //     run(shell);
    // }

    let shell = "cmd.exe".to_string();
    let _term = Term::new(shell, "".to_string());

    println!("Exiting");
}
