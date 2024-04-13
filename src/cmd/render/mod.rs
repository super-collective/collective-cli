mod evidence;
mod join_request;
mod members;

use crate::config::GlobalConfig;

#[derive(Debug, clap::Parser)]
pub struct RenderCommand {
	#[clap(subcommand)]
	subcommand: RenderSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum RenderSubCommand {
	Evidence(evidence::RenderEvidenceCommand),
	Members(members::RenderMembersCommand),
	JoinRequest(join_request::RenderJoinRequestCommand),
}

impl RenderCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		match &self.subcommand {
			RenderSubCommand::Evidence(c) => c.run(),
			RenderSubCommand::Members(c) => c.run(g),
			RenderSubCommand::JoinRequest(c) => c.run(),
		}
	}
}
