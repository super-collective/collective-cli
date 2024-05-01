// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{
	cmd::OutputArgs,
	collective::{fellowship::FellowshipEvidenceReport, potoc::PotocEvidenceReport},
	config::GlobalConfig,
	types::prelude::CollectiveId,
};
use schemars::schema_for;

#[derive(Debug, clap::Parser)]
pub struct SchemaEvidenceCommand {
	#[clap(flatten)]
	output: OutputArgs,
}

impl SchemaEvidenceCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		let schema = Self::schema(g);
		let schema = serde_json::to_string_pretty(&schema)?;
		let name = g.collective.nickname().replace(' ', "_").to_lowercase();
		let path = format!("{}.evidence.schema.json", name);

		self.output.write_schema(&path, &schema)
	}

	fn schema(g: &GlobalConfig) -> schemars::schema::RootSchema {
		match g.collective {
			CollectiveId::Fellowship => schema_for!(FellowshipEvidenceReport),
			CollectiveId::Potoc => schema_for!(PotocEvidenceReport),
		}
	}
}
