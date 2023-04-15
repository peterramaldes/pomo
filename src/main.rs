use clap::{Parser, Subcommand};

/// Pomo is responsible for track you daily tasks and help you to put this 
/// track time in your task managment application (like Jira)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start is responsible for start the tracking timing for one specific 
    /// description
    Start {
        /// Description provided to specific taks, i.e. Daily, Meeting, Task X
        #[arg(short, long)] // TODO: what is short, long?
        description: String,
    },
    /// Stop is responsible for stop the tracking timing for the current 
    /// tasking running
    Stop {
    },
}

fn main() {
    let cli = Cli::parse();
}
