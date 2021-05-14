extern crate clap;
use clap::{Arg, App};
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("mypipe")
                          .version("1.0")
                          .author("Groupe 7")
                          .about("Does awesome things")
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

    let input = matches.value_of("in").unwrap();
    let output = matches.value_of("out").unwrap();
    let commandInput = match Command::new("sh")
                            .arg("-c")
                            .arg(input)
                            .stdout(Stdio::piped())
                            .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };
    let commandOutput = Command::new("sh")
                            .arg("-c")
                            .arg(output)
                            .stdin(commandInput.stdout.unwrap())
                            .output()
                            .unwrap();
    let stdout = String::from_utf8(commandOutput.stdout).unwrap();
    let stderr = String::from_utf8(commandOutput.stderr).unwrap();
    println!("{}", stdout);
    println!("{}", stderr);



}
