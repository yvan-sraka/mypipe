use clap::{App, Arg};
use std::{
    io,
    process::{Command, Stdio},
};

fn main() -> io::Result<()> {
    let matches = App::new("MasterMind")
        .args(&[
            Arg::new("in")
                .long("in")
                .short('i')
                .about("Input")
                .required(true)
                .takes_value(true),
            Arg::new("out")
                .long("out")
                .short('o')
                .about("Output")
                .required(true)
                .multiple_occurrences(true)
                .takes_value(true),
        ])
        .get_matches();

    let in_command = matches.value_of("in").unwrap();
    let out_commands: Vec<_> = matches.values_of("out").unwrap().collect();

    let mut in_args = shlex::split(in_command)
        .unwrap_or_else(|| panic!("`--in` argument should be a valid command"));

    for i in 0..out_commands.len() {
        if shlex::split(out_commands[0].clone()).is_none() {
            panic!("`--out[{}]` argument should be a valid command", i)
        }
    }

    let mut command_child = Command::new(in_args.remove(0))
        .args(in_args)
        .stdout(Stdio::piped())
        .spawn()?;

    for out_command in out_commands {
        let mut out_args = shlex::split(out_command.clone()).unwrap();

        command_child = Command::new(out_args.remove(0))
            .args(out_args)
            .stdout(Stdio::piped())
            .stdin(command_child.stdout.unwrap())
            .spawn()?;
    }

    println!(
        "{}",
        String::from_utf8_lossy(&command_child.wait_with_output()?.stdout)
    );

    Ok(())
}
