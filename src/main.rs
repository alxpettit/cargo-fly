use std::env;
use std::process::{Command, Stdio};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    // Shhh, don't tell them :S
    if let Some(x) = args.iter_mut().find(|x| **x == "fly".to_string()) {
        *x = "run".to_string();
    }

    Command::new("cargo")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .expect("Failed to run cargo.")
        .wait()
        .expect("Isn't running :(");
}
