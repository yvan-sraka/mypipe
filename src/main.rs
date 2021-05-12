use structopt::StructOpt;
use std::process::Command;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Cli {
    // /// The pattern to look for
    // pattern: String,
    /// The path to the file to read
    // #[structopt(parse(from_os_str))]
    // path: std::path::PathBuf,
    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "i", long = "input", default_value = "Test")]
    input: String,

    #[structopt(short = "o", long = "output", default_value = "echo")]
    output: String,
}

fn main() {
    let args = Cli::from_args();
    let a = Command::new(args.input)
        .output()
        .expect("input command failed to start");

    let command_executed = std::str::from_utf8(a.stdout.as_slice()).unwrap().trim();

    Command::new(args.output)
        .arg(command_executed)
        .spawn()
        .expect("output command failed to start");

}
