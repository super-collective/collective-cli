// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{
	cmd::OutputArgs,
	collective::{fellowship::FellowshipJoinRequest, potoc::PotocJoinRequest},
	config::GlobalConfig,
	types::prelude::*,
};
use schemars::schema_for;

#[derive(Debug, clap::Parser)]
pub struct SchemaJoinRequestCommand {
	#[clap(flatten)]
	output: OutputArgs,
}

impl SchemaJoinRequestCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		let schema = Self::schema(g);
		let schema = serde_json::to_string_pretty(&schema)?;
		let name = g.collective.nickname().replace(' ', "_").to_lowercase();
		let path = format!("{}.join_request.schema.json", name);

		self.output.write_schema(&path, &schema)
	}

	fn schema(g: &GlobalConfig) -> schemars::schema::RootSchema {
		match g.collective {
			CollectiveId::Fellowship => schema_for!(FellowshipJoinRequest),
			CollectiveId::Potoc => schema_for!(PotocJoinRequest),
		}
	}
}
