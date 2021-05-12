use std::str;

use std::process::{Command, Stdio};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long = "--in")]
    input_command: Vec<String>,

    #[structopt(short, long = "--out")]
    output_command: Vec<String>,
}

fn main() {
    let args = Cli::from_args();
    let mut first_command = Command::new(&args.input_command[0]);
    let mut input_command = args.input_command;
    if input_command.len() > 1 {
        input_command.remove(0);
        first_command.args(input_command);
    }
    let output1 = first_command.stdout(Stdio::piped()).spawn();

    let mut second_command = Command::new(&args.output_command[0]);
    let mut output_command = args.output_command;
    if output_command.len() > 1 {
        output_command.remove(0);
        second_command.args(output_command);
    }
    let output2 = second_command
        .stdin(Stdio::from(output1.unwrap().stdout.unwrap()))
        .output()
        .ok();
    println!("{}", str::from_utf8(&output2.unwrap().stdout).unwrap());
}
