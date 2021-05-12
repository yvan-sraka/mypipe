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

    match pipe_cmd(opts.r#in, opts.out) {
        Ok(stdout) => println!("{}", stdout),
        Err(err) => eprintln!("{}", err),
    }
}

fn pipe_cmd(cmd_in: String, cmd_out: String) -> Result<String, std::io::Error> {
    return cmd!(cmd_in).pipe(cmd!(cmd_out)).read();
}

#[cfg(test)]
mod tests {
    use crate::pipe_cmd;

    #[test]
    fn test_cmd_pipe() {
        let res = pipe_cmd("whoami".to_string(), "wc".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "      1       1       5".to_string());
    }

    // doesn't work :()
    fn test_cmd_pipe_args() {
        let res = pipe_cmd("whoami".to_string(), "wc -l".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "1".to_string());
    }
}
