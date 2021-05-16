use clap::{App, Arg};
use std::{ process::{Command, Stdio}, io};

fn main() -> io::Result<()> {
    let matche = App::new("Pipeline")
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

    let my_input = matche.value_of("in").unwrap();
    let my_output = matche.value_of("out").unwrap();

    let input_output = Command::new(my_input).stdout(Stdio::piped()).spawn()?;
    let output_output = Command::new(my_output).stdin(input_output.stdout.unwrap()).output()?;

    println!("{}", String::from_utf8_lossy(&output_output.stdout));

    Ok(())
}  