use std::env;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::process::Command;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    cmd: String
}

#[derive(Debug)]
struct CustomError(String);

fn main()   {
    let args = Cli::from_args();
    let status = Command::new(args.cmd).status().expect("failed to execute process");


    match status.code() {
        Some(value) => return,
        None => pien(),
    }
}

fn pien() {
    let mut path = env::home_dir().unwrap();
    path.push("pien.mp3");
    Command::new("afplay").arg(path).status().expect("failed to execute process");
}
