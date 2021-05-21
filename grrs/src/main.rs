use structopt::StructOpt;
use log::{info, warn};

#[derive(StructOpt)]
struct Cli {
    pattern1: String,
    pattern2: String,
}

fn main() {
// let result = std::fs::read_to_string("test.txt").unwrap();
    let pattern1 = std::env::args().nth(1).expect("no pattern given");
    let pattern2 = std::env::args().nth(2).expect("no pattern given");
    let args = Cli {
        pattern1: pattern1,
        pattern2: pattern2,
    };
    match result {
        Ok(content) => { println!("File content: {}", content); }
        Err(error) => { println!("Oh noes: {}", error); }
    }
    let args = Cli::from_args();
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}
