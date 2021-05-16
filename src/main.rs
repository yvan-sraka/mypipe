use clap::{Arg, App};
use std::io::{self, Write};

fn main() {
    let matches = App::new("mypipe")
        .version("0.1.0")
        .author(" rema ")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("in")
            .long("in")
            .takes_value(true)
            .help("input of pipe"))
        .arg(Arg::with_name("out")
            .long("out")
            .takes_value(true)
            .help("ouput of pipe"))
        .get_matches();

    let input = matches.value_of("in").unwrap();
    let output = matches.value_of("out").unwrap();
    let arg = "-c";

    // executing left side command
    let left_cmd_processed = std::process::Command::new("sh")
        .arg(arg)
        .arg(input)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("error");

    let right_cmd_processed = std::process::Command::new("sh")
        .arg(arg)
        .arg(output)
        .stdin(std::process::Stdio::from(Some(left_cmd_processed).map_or(std::process::Stdio::inherit(),
            | child_proc: std::process::Child | std::process::Stdio::from(child_proc.stdout.unwrap()))))
        .output()
        .expect("error");

    io::stdout().write_all(&right_cmd_processed.stdout).unwrap();
}