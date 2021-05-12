fn main() {
    // Loading all args
    let args: Vec<String> = std::env::args().collect();

    // Check arg number
    if args.len() != 5 {
        panic!("Illegal argument size");
    }

    //Loading commands to execute
    let input = args.get(2).expect("Failed to load input command");
    let ouput = args.get(4).expect("Failed to load ouput command");

    // println!("{}", input); // Debug print
    // println!("{}", ouput); // Debug print

    let command1 = std::process::Command::new(input)
        .output()
        .expect("input command failed");

    let res = std::str::from_utf8(command1.stdout.as_slice()).expect("failed to execute command1").trim();
    // println!("{}", res); // Debug print

    // Run ouput command with first the result in argument
    let command2 = std::process::Command::new(ouput)
        .arg(res)
        .output()
        .expect("input command failed");

    let final_output = std::str::from_utf8(command2.stdout.as_slice()).expect("failed to execute command2");

    println!("{}", final_output);
}

