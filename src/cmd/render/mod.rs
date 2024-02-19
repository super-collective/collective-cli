mod evidence;

#[derive(Debug, clap::Parser)]
pub struct RenderCommand {
	#[clap(subcommand)]
	subcommand: RenderSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum RenderSubCommand {
	Evidence(evidence::RenderEvidenceCommand),
}

impl RenderCommand {
	pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		match &self.subcommand {
			RenderSubCommand::Evidence(c) => c.run(),
		}
	}
}
