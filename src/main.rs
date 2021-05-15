use std::str;

use std::process::{Command, Stdio};
use structopt::StructOpt;

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
    if_str_command_more_1_add_args(args.input_command, &mut first_command);
    let output1 = first_command.stdout(Stdio::piped()).spawn();

    let mut second_command = Command::new(&args.output_command[0]);
    if_str_command_more_1_add_args(args.output_command, &mut second_command);
    let output2 = second_command
        .stdin(Stdio::from(output1.unwrap().stdout.unwrap()))
        .output()
        .ok();
    println!("{}", str::from_utf8(&output2.unwrap().stdout).unwrap());
}

fn if_str_command_more_1_add_args(mut str_command: Vec<String>, command: &mut Command) {
    if str_command.len() > 1 {
        str_command.remove(0);
        command.args(str_command);
    }
}