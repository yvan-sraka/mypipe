use clap::{App, Arg, ArgMatches};
use std::process::{Command, Stdio};

fn main() {
    let matches = init_app();
    let command = Command::new(matches.value_of("in").unwrap())
    .stdout(Stdio::piped())
    .spawn();
    let result = match command {
        Err(e) => panic!("{}", e),
        Ok(process) => process,
    };
    let output = Command::new(matches.value_of("out").unwrap())
    .stdin(result.stdout.unwrap())
    .output()
    .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    println!("Output : {}", output);

fn init_app() -> ArgMatches<'static> {
    App::new("mypipe")
        .version("1.0")
        .author("Hervé HUANG <hhuang4@myges.com>")
        .about("Get std input and return std output")
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .value_name("INPUT")
                .help("Define your input")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .value_name("OUTPUT")
                .help("Define your output")
                .takes_value(true)
                .required(true)
        )
        .get_matches()
}
