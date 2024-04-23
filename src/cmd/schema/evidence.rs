// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{cmd::OutputArgs, collective::fellowship::FellowshipEvidenceReport};

#[derive(Debug, clap::Parser)]
pub struct SchemaEvidenceCommand {
	#[clap(flatten)]
	output: OutputArgs,
}

impl SchemaEvidenceCommand {
	pub fn run(&self) -> anyhow::Result<()> {
		let schema = FellowshipEvidenceReport::schema();

		self.output.write_schema("evidence.schema.json", schema)
	}
}
