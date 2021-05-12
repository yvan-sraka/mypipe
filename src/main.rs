use clap::{App, Arg};
use std::{
    io,
    process::{Child, Command, Stdio},
};

fn is_valid_command(command: &str) -> Result<(), String> {
    match shlex::split(command) {
        None => Err(format!("invalid command")),
        _ => Ok(()),
    }
}

fn spawn<T>(command: &str, stdin: T) -> io::Result<Child>
where
    T: Into<Stdio>,
{
    let mut args = shlex::split(command.clone()).unwrap();

    Command::new(args.remove(0))
        .args(args)
        .stdout(Stdio::piped())
        .stdin(stdin.into())
        .spawn()
}

fn main() -> io::Result<()> {
    let matches = App::new("Piper")
        .args(&[
            Arg::new("in")
                .long("in")
                .short('i')
                .about("Input command")
                .required(true)
                .takes_value(true)
                .validator(is_valid_command),
            Arg::new("out")
                .long("out")
                .short('o')
                .about("Output command")
                .required(true)
                .takes_value(true)
                .multiple_occurrences(true)
                .validator(is_valid_command),
        ])
        .get_matches();

    let in_command = matches.value_of("in").unwrap();

    let mut command_child = spawn(in_command, Stdio::inherit())?;

    for out_command in matches.values_of("out").unwrap() {
        command_child = spawn(out_command, command_child.stdout.unwrap())?;
    }

    println!(
        "{}",
        String::from_utf8_lossy(&command_child.wait_with_output()?.stdout)
    );

    Ok(())
}
