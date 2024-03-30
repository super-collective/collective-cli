mod evidence;
mod join_request;

use crate::config::GlobalConfig;

#[derive(Debug, clap::Parser)]
pub struct NewCommand {
	#[clap(subcommand)]
	subcommand: NewSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum NewSubCommand {
	/// Create a new evidence report.
	Evidence(evidence::NewEvidenceCommand),
	// Create a new member request.
	JoinRequest(join_request::NewJoinRequestCommand),
}

impl NewCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		match &self.subcommand {
			NewSubCommand::Evidence(c) => c.run(),
			NewSubCommand::JoinRequest(c) => c.run(g),
		}
	}
}
