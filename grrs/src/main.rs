use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
