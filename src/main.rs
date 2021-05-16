use std::env;
use std::process::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    //
    // println!("{}", args.len());
    if args.len() != 5 {
        println!("Wrong arguments number");
        return;
    }

    let arg_in = &args[2];
    let arg_out = &args[4];

    // println!("{}", arg_in);
    // println!("{}", arg_out);

    let first_command = Command::new(arg_in)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Argument in not recognized");

    Command::new(arg_out)
        .stdin(first_command.stdout.unwrap())
        .spawn()
        .expect("First command result incorrect");
}
