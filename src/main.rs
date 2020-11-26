use std::{env, process};

fn main() {
    let running_os = env::consts::OS;

    if running_os != "linux" {
        eprintln!("Application error: Linkus can only be run on Linux systems");
        process::exit(1);
    }
}
