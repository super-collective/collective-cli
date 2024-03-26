use crate::{cmd::OutputArgs, collective::fellowship::FellowshipEvidenceReport};

#[derive(Debug, clap::Parser)]
pub struct SchemaEvidenceCommand {
	#[clap(flatten)]
	output: OutputArgs,
}

impl SchemaEvidenceCommand {
	pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		let schema = FellowshipEvidenceReport::schema();

		self.output.write_schema("evidence.schema.json", schema)
	}
}
