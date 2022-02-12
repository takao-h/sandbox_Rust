use clap::{App, Arg};

fn main() {
    let mtches =App::new("echor")
    .version("0.1.0")
    .author("takao-h")
    .about("Rust echo")
    .arg(
        Arg::with_name("text")
        .value_name("TEXT")
        .help("Input")
        .required(true)
        .min_values(1),
    )
    .arg(
        Arg::with_name("omit_newline")
        .short("n")
        .help("Do not print newline")
        .text_value(false),
    )
    .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}