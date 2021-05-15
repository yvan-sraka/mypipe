extern crate clap;
use clap::{Arg, App};
use figlet_rs::FIGfont;
extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

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

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let in_opt = matches.value_of("in").expect("This can't be None, we said it was required");
    let out_opt = matches.value_of("out").expect("This can't be None !");

    match out_opt {
        "figlet" => figlet(in_opt),
        "cowsay" => cowsay(in_opt),
        _ => { println!("ASCII Arts no content") },
    }

}

fn figlet(in_opt: &str) {
    let small_font = FIGfont::from_file("resources/small.flf").unwrap();
    let figure = small_font.convert(in_opt);
    println!("{}", figure.unwrap());
}

fn cowsay(in_opt: &str) {
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(in_opt.to_string().as_ref(), width, &mut writer).unwrap();
}
