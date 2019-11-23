use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The pattern to look for
    #[structopt(short = "p", long = "pattern")]
    pattern: String,
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    #[structopt(short = "f", long = "file")]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file {:?}", &args.path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
