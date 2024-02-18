mod evidence;

#[derive(Debug, clap::Parser)]
pub struct NewCommand {
    #[clap(subcommand)]
    subcommand: NewSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum NewSubCommand {
    /// Create a new evidence report.
    Evidence(evidence::NewEvidenceCommand),
}

impl NewCommand {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcommand {
            NewSubCommand::Evidence(c) => c.run(),
        }
    }
}
