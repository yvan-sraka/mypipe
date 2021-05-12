use std::process::Command;
use figlet_rs::FIGfont;


/// Search for a pattern in a file and display the lines that contain it.
use clap::{Arg, App};
use std::borrow::Cow;
use std::ops::Deref;

fn displayFigletOutput(output: Cow<str>) {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert(output.deref());
    println!("{}", figure.unwrap());
}

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Louis D. <louisdumontbarbosa@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::new("INPUT")
            .short('i')
            .long("in")
            .about("Sets an input command to process")
            .required(true).takes_value(true))
        .arg(Arg::new("OUTPUT")
            .short('o')
            .long("out")
            .about("Sets the ouput mode").required(true).takes_value(true)
        ).get_matches();
    if let Some(i) = matches.value_of("INPUT") {
        // Spawn a process, wait for it to finish, and collect it's output
        let the_output = Command::new(i).output()
            .ok().expect("Failed to execute.");
        // Encode the resulting data.
        let encoded = String::from_utf8_lossy(the_output.stdout.as_slice());
        if let Some(j) = matches.value_of("OUTPUT") {
            let result = match j {
                "figlet" => displayFigletOutput(encoded),
                _ => panic!("Not a valid output argument")
            };
        }
    }
    println!("Hello, world!");
}
