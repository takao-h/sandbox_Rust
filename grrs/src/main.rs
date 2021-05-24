use structopt::StructOpt;
use log::{info, warn};

#[derive(StructOpt)]
struct Cli {
    pattern1: String,
    pattern2: String,
}

fn main() {
// let result = std::fs::read_to_string("test.txt").unwrap();
// match result {
//     Ok(content) => { println!("File content: {}", content); }
//     Err(error) => { println!("Oh noes: {}", error); }
// }
    let pattern1 = std::env::args().nth(1).expect("no pattern given");
    let pattern2 = std::env::args().nth(2).expect("no pattern given");
    let args = Cli {
        pattern1: pattern1,
        pattern2: pattern2,
    };
    let args = Cli::from_args();
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
