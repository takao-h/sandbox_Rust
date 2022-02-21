use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
  file: Vec<String>,
  number_lines: bool,
  number_nonblank_listen: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
    .version("0.8.1")
    .author("takao-h")
    .about("Rust cat")
    .arg(
      Arg::with_name("files")
      .value_name("FILE")
      .help("Input files")
      .multiple(true)
      .default_value("-"),
    )
    .arg(
      Arg::with_name("number_nonblank")
      .short("b")
      .long("number")
      .help("Number lines")
      .takes_value(false)
      .conflicts_with("number_nonblank")
    )
    .get_matches();

    Ok(Config {
      file: matches.value_of_lossy("files").unwrap(),
      number_lines: matches.is_present("number"),
      number_nonblank_listen: matches.is_present("number_nonblank"),
    })
}

fn run (config: Config) -> MyResult {
  for filename in config.files {
    match open(&filename) {
      Err(e) => eprint!{"{}, {}", filename, e},
      Ok(file!) => {
        let mut last_num = 0;
        for (line_name, line_result) in file.lines().enumerate() {
            let line = line_result?;
            if config.number_lines {
              println!("{:6}\t{}", line_num + 1, line);
            } else if config.number_nonblank_lines {
              if !line.is_empty() {
                last_num += 1;
                println!("{:6}\t{}", last_num, line);
              } else {
                  println!();
              }
            } else {
                println!("{}", line);
            }
        }
      }
  }
  Ok(())
}

fn open(filename: &str) -> MyResult {
  match filename {
      "-" => Ok(Box::new(BufReader::new(io::stdin()))),
      _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}