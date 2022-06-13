use std::env;
use std::process;
use std::process::{Command, Stdio};
use std::ffi::OsString;

fn main() {
    let mut args = env::args_os().peekable();

    // we either have 1 extra arg for the executable name
    // or 2 extra args if called as `cargo drive`
    args.next();
    if args.peek() == Some(&OsString::from("drive")) {
        args.next();
    }

    let status = Command::new("cargo")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .arg("run")
        .args(args)
        .status()
        .expect("failed to execute cargo");

    match status.code() {
        Some(code) => process::exit(code),
        None => process::exit(0)
    };
}
