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
                .takes_value(true),
        ])
        .get_matches();

    let in_command = matches.value_of("in").unwrap();
    let out_command = matches.value_of("out").unwrap();

    let mut in_args = shlex::split(in_command)
        .unwrap_or_else(|| panic!("`--in` argument should be a valid command"));
    let mut out_args = shlex::split(out_command)
        .unwrap_or_else(|| panic!("`--out` argument should be a valid command"));

    let in_command_child = Command::new(in_args.remove(0))
        .args(in_args)
        .stdout(Stdio::piped())
        .spawn()?;

    let out_command_output = Command::new(out_args.remove(0))
        .args(out_args)
        .stdin(in_command_child.stdout.unwrap())
        .output()?;

    println!("{}", String::from_utf8_lossy(&out_command_output.stdout));

    Ok(())
}
