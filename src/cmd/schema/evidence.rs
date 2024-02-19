use crate::{cmd::OutputArgs, fellowship::FellowshipReport};

#[derive(Debug, clap::Parser)]
pub struct SchemaEvidenceCommand {
	#[clap(flatten)]
	output: OutputArgs,
}

impl SchemaEvidenceCommand {
	pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		let schema = FellowshipReport::schema();

		self.output.write_schema(schema)
	}
}
