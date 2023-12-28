use clap::{Parser, Subcommand};
use pragrat::genast::generate_ast;

#[derive(Subcommand, Debug)]
enum Generate {
    /// Generate modules used to parse the AST in pragrat.
    Ast,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate {
        #[clap(subcommand)]
        commmand: Generate,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of source code directory
    #[arg(short, long, default_value = ".")]
    path: String,

    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Generate { ref commmand } => match commmand {
            Generate::Ast => generate_ast(&args.path),
        },
    }
    dbg!("{}", args);
}
