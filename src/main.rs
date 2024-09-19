mod build;
use build::{cargo_build, BuildType};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Overlay {
        #[arg(short, long)]
        cross: bool,

        #[arg(short, long)]
        package: Option<String>,

        #[arg(short, long)]
        target: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Overlay {
            cross,
            package,
            target,
            ..
        } => cargo_build(
            package.as_deref(),
            target.as_deref(),
            if cross {
                BuildType::Cross
            } else {
                BuildType::Cargo
            },
        ),
    };
}
