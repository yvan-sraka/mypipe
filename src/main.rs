use clap::{App, Arg};
use std::{ process::{Command, Stdio}, io, char };

fn main() -> io::Result<()> {
    let matche = App::new("My Pipe Tool")
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

    let input = matche.value_of("in").unwrap();
    let output = matche.value_of("out").unwrap();

    let input_output = Command::new(input).stdout(Stdio::piped()).spawn()?;
    let output_output = Command::new(output).stdin(input_output.stdout.unwrap()).output()?;

    println!("{}", String::from_utf8_lossy(&output_output.stdout));

    Ok(())
} 