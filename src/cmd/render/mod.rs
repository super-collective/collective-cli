mod evidence;
mod members;
mod join_request;

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
}

impl RenderCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		match &self.subcommand {
			RenderSubCommand::Evidence(c) => c.run(),
			RenderSubCommand::Members(c) => c.run(g),
		}
	}
}
