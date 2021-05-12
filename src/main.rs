use clap::{App, Arg};
use std::process::Command;
use std::str;
use figlet_rs::FIGfont;


fn main() {
    let matches = App::new("My Super Useless Program")
        .version("1.0")
        .author("Vagahbond <firroloni.yoni@gmail.com>")
        .about("Does useless things")
        .arg(
            Arg::new("command")
                .about("Command of which we will dpslay the result")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("figlet")
                .short('f')
                .long("figlet")
                .about("flag to apply figlet")
                .required(false)
        )
        .get_matches();

    if let Some(c) = matches.value_of("command") {
        let output = Command::new(c)
        .output()
        .expect("Failed to execute command");

        let stdout = str::from_utf8(&output.stdout).expect("Could not parse command result.");

        if matches.occurrences_of("figlet") > 0 {
            let standard_font = FIGfont::standand().unwrap();
            let figure = standard_font.convert(stdout);
            println!("{}", figure.expect("Could not apply figlet."));
        } else {
            println!("{}", stdout);
        }
        
        
        

    } else {
        panic!("Unprocessable command argument.");
    }


}
