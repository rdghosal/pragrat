use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of source code directory
    #[arg(short, long, default_value = ".")]
    path: String,
}

fn main() {
    let args = Args::parse();
    dbg!("{}", args);
}
