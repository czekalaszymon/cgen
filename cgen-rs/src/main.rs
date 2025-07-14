use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        project_name: Option<String>,
        #[arg(long)]
        no_clang_format: bool,
    },
    Build,
    Run,
    Clean,
    Format,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { project_name, no_clang_format } => {
            commands::new::run(project_name, !no_clang_format)?;
        }
        Commands::Build => commands::build::run()?,
        Commands::Run => commands::run::run()?,
        Commands::Clean => commands::clean::run()?,
        Commands::Format => commands::format::run()?,
    }

    Ok(())
}
