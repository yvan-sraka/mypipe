use std::process::{Command, Output};
use std::str;

use clap::{App, Arg};

fn fork(cmd: &str, args: &[&str]) -> Output {
    Command::new(cmd)
        .args(args)
        .output()
        .expect("failed to execute process")
}

fn main() {
    let matches = App::new("MyPipe")
        .arg(
            Arg::new("in")
                .long("in")
                .short('i')
                .about("Input command")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("out")
                .long("out")
                .short('o')
                .about("Output command")
                .required(true)
                .takes_value(true)
        )
        .get_matches();

    let input = matches.value_of("in").unwrap();
    let output = matches.value_of("out").unwrap();

    let command1 = fork(input, &[]);

    let command1_output = str::from_utf8(command1.stdout.as_slice()).expect("failed to parse output");

    let command2 = fork(output, &[command1_output]);

    let command2_output = str::from_utf8(command2.stdout.as_slice()).expect("failed to parse output");

    println!("{}", command2_output);
}
