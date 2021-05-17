extern crate clap;
use std::process::{Command, Stdio};
use clap::{Arg, App};


fn main() {
    let matches = App::new("mypipe")
    .version("1.0")
    .arg(Arg::with_name("in")
    .long("in")
       .help("Sets the input to use")
       .takes_value(true)
       .required(true))
    .arg(Arg::with_name("out")
    .long("out")
         .help("Sets the output to use")
         .takes_value(true)
         .required(true))
    .get_matches();

    //println!("Command :  input {:?}, output {:?}", matches.value_of("in").unwrap(), matches.value_of("out"));

    let input = matches.value_of("in").unwrap();
    let output = matches.value_of("out").unwrap();


    let command_input = match Command::new("sh")
        .arg("-c")
        .arg(input)
        .stdout(Stdio::piped())
        .spawn() {
            Err(error) => panic!("Error : {}", error),
            Ok(process) => process,
        };
    let command_output = Command::new("sh")
        .arg("-c")
        .arg(output)
        .stdin(command_input.stdout.unwrap())
        .output()
        .unwrap();
    
    let stdout = String::from_utf8(command_output.stdout).unwrap();
    println!("{}", stdout);
}
