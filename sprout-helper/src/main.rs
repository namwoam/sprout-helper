use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new course
    New {
        /// course name
        #[arg(short, long)]
        name: String,
    },
    /// create a new module
    Add {
        // module name
        #[arg(short, long)]
        name: String,
        #[arg(value_enum, short, long)]
        module_type: ModuleTypes,
    },
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ModuleTypes {
    Slide,
    CodingProblem,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New { name } => {}

        Commands::Add { name, module_type } => {}
    }
}
