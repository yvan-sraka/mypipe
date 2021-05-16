use clap::{App, Arg};
use std::{ process::{Command, Stdio}, io, char };

fn main() -> io::Result<()> {
    let matche = App::new("pipe test").args
        (&[
            Arg::new("one")
                .long("one")
                .short('o')
                .about("first argument")
                .required(true)
                .takes_value(true),
            Arg::new("two")
                .long("two")
                .short('t')
                .about("second argument")
                .required(true)
                .takes_value(true),
        ]).get_matches();

    let one = matche.value_of("one").unwrap();
    let two = matche.value_of("two").unwrap();
    let input_one_to_output_one = Command::new(one).stdout(Stdio::piped()).spawn()?;
    let result = Command::new(two).stdin(input_one_to_output_one.stdout.unwrap()).output()?;

    println!("{}", String::from_utf8_lossy(&result.stdout));

    Ok(())
}
