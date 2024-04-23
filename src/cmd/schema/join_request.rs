// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{cmd::OutputArgs};
use crate::types::prelude::*;
use schemars::schema_for;
use crate::config::GlobalConfig;
use crate::collective::fellowship::FellowshipJoinRequest;
use crate::collective::potoc::PotocJoinRequest;

#[derive(Debug, clap::Parser)]
pub struct SchemaJoinRequestCommand {
	#[clap(flatten)]
	output: OutputArgs,
}

impl SchemaJoinRequestCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		let schema = Self::schema(g);
		let schema = serde_json::to_string_pretty(&schema)?;

		self.output.write_schema("join_request.schema.json", &schema)
	}

	fn schema(g: &GlobalConfig) -> schemars::schema::RootSchema {
		match g.collective {
			CollectiveId::Fellowship => schema_for!(FellowshipJoinRequest),
			CollectiveId::Potoc => schema_for!(PotocJoinRequest),
		}
	}
}
