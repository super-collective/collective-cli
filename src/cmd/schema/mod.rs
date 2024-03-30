mod evidence;

#[derive(Debug, clap::Parser)]
pub struct SchemaCommand {
	#[clap(subcommand)]
	subcommand: SchemaSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum SchemaSubCommand {
	/// Generate a schema of an evidence report.
	Evidence(evidence::SchemaEvidenceCommand),
}

impl SchemaCommand {
	pub fn run(&self) -> anyhow::Result<()> {
		match &self.subcommand {
			SchemaSubCommand::Evidence(c) => c.run(),
		}
	}
}
