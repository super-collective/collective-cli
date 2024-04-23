mod evidence;
mod join_request;

use crate::config::GlobalConfig;

#[derive(Debug, clap::Parser)]
pub struct SchemaCommand {
	#[clap(subcommand)]
	subcommand: SchemaSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum SchemaSubCommand {
	/// Generate a schema of an evidence report.
	Evidence(evidence::SchemaEvidenceCommand),
	JoinRequest(join_request::SchemaJoinRequestCommand),
}

impl SchemaCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		match &self.subcommand {
			SchemaSubCommand::Evidence(c) => c.run(),
			SchemaSubCommand::JoinRequest(c) => c.run(g),
		}
	}
}
