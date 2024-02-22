mod evidence;

#[derive(Debug, clap::Parser)]
pub struct IndexCommand {
	#[clap(subcommand)]
	subcommand: IndexSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum IndexSubCommand {
	/// Create some evidence Index.
	Evidence(evidence::IndexEvidenceCommand),
}

impl IndexCommand {
	pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		match &self.subcommand {
			IndexSubCommand::Evidence(c) => c.run(),
		}
	}
}
