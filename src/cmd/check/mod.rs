mod evidence;

#[derive(Debug, clap::Parser)]
pub struct CheckCommand {
	#[clap(subcommand)]
	subcommand: CheckSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum CheckSubCommand {
	/// Check one or more evidence reports for formatting errors.
	Evidence(evidence::CheckEvidenceCommand),
}

impl CheckCommand {
	pub fn run(&self) -> anyhow::Result<()> {
		match &self.subcommand {
			CheckSubCommand::Evidence(c) => c.run(),
		}
	}
}
