use clap::App;
use duct::cmd;

fn main() {
    let matches = App::new("mypipe")
        .version("1.0")
        .about("Pipe one command into another")
        .arg("-i, --in=[COMMAND] 'command'")
        .arg("-o, --out=[COMMAND] 'command'")
        .get_matches();

    let stdout = cmd!(matches.value_of("in").unwrap())
        .pipe(cmd!(matches.value_of("out").unwrap()))
        .read()
        .unwrap();

    println!("{}", stdout);
}
