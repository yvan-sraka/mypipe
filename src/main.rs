use clap::{Arg, App};

fn main() {
    let matches = App::new("mypipe")
        .version("0.1.0")
        .author(" rema ")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("in")
            .long("in")
            .takes_value(true)
            .help("input of pipe"))
        .arg(Arg::with_name("out")
            .long("out")
            .takes_value(true)
            .help("ouput of pipe"))
        .get_matches();

    let input = matches.value_of("in").unwrap();
    let output = matches.value_of("out").unwrap();

    println!("in: {}", input);
    println!("out: {}", output);
}