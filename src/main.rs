use clap::Clap;
use duct::cmd;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(short, long)]
    r#in: String,
    #[clap(short, long)]
    out: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match cmd!(opts.r#in).pipe(cmd!(opts.out)).read() {
        Ok(stdout) => println!("{}", stdout),
        Err(err) => eprintln!("{}", err),
    }
}
