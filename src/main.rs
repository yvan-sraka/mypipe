extern crate clap;
use clap::{Arg, App};
use std::process::{Command, Stdio};

fn main() {
    let app = App::new("mypipe")
        .version("1.0")
        .author("Nicolas GARNIER & Yanis TOUAHRI ");

    let input_option = Arg::with_name("in")
        .long("in")
        .value_name("input")
        .help("Give me input")
        .required(true)
        .takes_value(true);

    let output_option = Arg::with_name("out")
        .long("out")
        .value_name("output")
        .help("Give me output")
        .required(true)
        .takes_value(true);

    let app = app.arg(input_option).arg(output_option);

    let matches = app.get_matches();

    let in_opt = matches.value_of("in").unwrap();
    let out_opt = matches.value_of("out").unwrap();

    let in_cmd = match Command::new("sh")
        .arg("-c")
        .arg(in_opt)
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("{}", why),
        Ok(process) => process,
    };

    let out_cmd = Command::new("sh")
        .arg("-c")
        .arg(out_opt)
        .stdin(in_cmd.stdout.unwrap())
        .output()
        .unwrap();

    let stdout = String::from_utf8(out_cmd.stdout).unwrap();
    let stderr = String::from_utf8(out_cmd.stderr).unwrap();

    println!("{}", stdout);
    println!("{}", stderr);
}
