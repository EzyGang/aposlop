use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "aposlop",
    version,
    about = "Detect duplicate code and report cyclomatic complexity"
)]
struct Cli;

fn main() {
    let _cli = Cli::parse();
}
