use clap::{load_yaml, App};
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from(yml).get_matches();

    let mut input_command: Option<std::process::Output> = None;
    let output_command: std::process::Output;

    if let Some(i) = matches.value_of("INPUT") {
        input_command = Some(Command::new(i).output().expect("failed to execute process"));
    }

    if let Some(i) = matches.value_of("OUTPUT") {
        if let Some(value) = input_command {
            output_command = Command::new(i)
                .args(String::from_utf8(value.stdout))
                .output()
                .expect("failed to execute process");
            io::stdout().write_all(&output_command.stdout).unwrap();
        }
    }
}
