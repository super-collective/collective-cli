mod evidence;
mod join_request;

use crate::config::GlobalConfig;

#[derive(Debug, clap::Parser)]
pub struct CheckCommand {
	#[clap(subcommand)]
	subcommand: CheckSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum CheckSubCommand {
	/// Check one or more evidence reports for formatting errors.
	Evidence(evidence::CheckEvidenceCommand),
	/// Check a join-request for formatting errors.
	JoinRequest(join_request::CheckJoinRequestCommand),
}

impl CheckCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		match &self.subcommand {
			CheckSubCommand::Evidence(c) => c.run(g),
			CheckSubCommand::JoinRequest(c) => c.run(g),
		}
	}
}
