use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "galed",
    about = "גל-עד — a requirements definition language for human-AI hybrid authorship in regulated systems",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate .gal files against the schema and field lock rules
    Validate {
        /// Path to a .gal file or .galed/ directory
        #[arg(default_value = ".")]
        path: String,
    },
    /// Build and display the requirement graph
    Graph {
        /// Output format: json, dot, or summary
        #[arg(short, long, default_value = "summary")]
        format: String,
        /// Path to .galed/ directory
        #[arg(default_value = ".")]
        path: String,
    },
    /// Open a proposal to change a field value
    Propose {
        /// Requirement ID (e.g. REQ-001)
        req_id: String,
        /// Field path (e.g. ac.threshold)
        field: String,
        /// Proposed value
        value: String,
        /// Rationale for the change
        #[arg(short, long)]
        rationale: Option<String>,
    },
    /// Show open proposals, unresolved conflicts, and impact flags
    Status {
        /// Path to .galed/ directory
        #[arg(default_value = ".")]
        path: String,
    },
    /// Initialize a .galed/ directory in the current project
    Init {
        /// Project name
        #[arg(short, long)]
        name: Option<String>,
        /// Domain (clinical, aerospace, research, general)
        #[arg(short, long, default_value = "general")]
        domain: String,
    },
    /// Import a requirement from an external source
    Import {
        /// Source type (jira, ado)
        #[arg(long)]
        source: String,
        /// Issue identifier
        #[arg(long)]
        issue: String,
        /// Domain for the imported requirement
        #[arg(long, default_value = "general")]
        domain: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { path } => {
            println!("galed validate — not yet implemented");
            println!("target: {}", path);
        }
        Commands::Graph { format, path } => {
            println!("galed graph — not yet implemented");
            println!("format: {}, path: {}", format, path);
        }
        Commands::Propose { req_id, field, value, rationale } => {
            println!("galed propose — not yet implemented");
            println!("req: {}, field: {}, value: {}", req_id, field, value);
            if let Some(r) = rationale {
                println!("rationale: {}", r);
            }
        }
        Commands::Status { path } => {
            println!("galed status — not yet implemented");
            println!("path: {}", path);
        }
        Commands::Init { name, domain } => {
            println!("galed init — not yet implemented");
            println!("name: {:?}, domain: {}", name, domain);
        }
        Commands::Import { source, issue, domain } => {
            println!("galed import — not yet implemented");
            println!("source: {}, issue: {}, domain: {}", source, issue, domain);
        }
    }
}
