
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Decoration test")
                          .version("1.0")
                          .author("Jamal SJ")
                          .about("try to use pipe")
                          .arg(Arg::with_name("input")
                               .short("in")
                               .long("input")
                               .value_name("input")
                               .help("Make an input value")
                               .takes_value(true))
                          .arg(Arg::with_name("output")
                                .short("out")
                                .long("output")
                                .value_name("output")
                                .help("Select a decorator for output")
                                .takes_value(true))
                          .get_matches();

    let input = matches.value_of("input").unwrap().to_string();

    match matches.value_of("output").unwrap(){
        "parentheses" => println!("(( {} ))", input),
        "tiret" => println!("-- {} --", input),
        _ => println!("{}", input),
    }
}