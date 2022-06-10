use std::env;
use std::process;
use std::process::{Command, Stdio};

fn main() {
    let mut args = env::args_os();

    let status = Command::new("cargo")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .arg("run")
        .args(args.next())
        .status()
        .expect("failed to execute cargo");

    match status.code() {
        Some(code) => process::exit(code),
        None => process::exit(0)
    };
}
