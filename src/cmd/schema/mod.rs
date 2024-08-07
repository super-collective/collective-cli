// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

mod evidence;
mod join_request;

use crate::{
	cmd::OutputArgs,
	config::GlobalConfig,
	types::prelude::{EvidenceReport, JoinRequest},
};
use schemars::schema_for;

#[derive(Debug, clap::Parser)]
pub struct SchemaCommand {
	#[clap(subcommand)]
	subcommand: SchemaSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum SchemaSubCommand {
	/// Generate a schema of an evidence report.
	Evidence(evidence::SchemaEvidenceCommand),
	GenericEvidence(GenericSchemaEvidence),

	JoinRequest(join_request::SchemaJoinRequestCommand),
	GenericJoinRequest(GenericSchemaJoinRequest),
}

#[derive(Debug, clap::Parser)]
pub struct GenericSchemaEvidence {
	#[clap(flatten)]
	output: OutputArgs,
}

#[derive(Debug, clap::Parser)]
pub struct GenericSchemaJoinRequest {
	#[clap(flatten)]
	output: OutputArgs,
}

impl SchemaCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		match &self.subcommand {
			SchemaSubCommand::Evidence(c) => c.run(g),
			SchemaSubCommand::GenericEvidence(c) => c.run(),
			SchemaSubCommand::JoinRequest(c) => c.run(g),
			SchemaSubCommand::GenericJoinRequest(c) => c.run(),
		}
	}
}

impl GenericSchemaEvidence {
	pub fn run(&self) -> anyhow::Result<()> {
		let schema = schema_for!(EvidenceReport);
		let schema = serde_json::to_string_pretty(&schema)?;

		let path = "evidence.schema.json";
		self.output.write_schema(&path, &schema)
	}
}

impl GenericSchemaJoinRequest {
	pub fn run(&self) -> anyhow::Result<()> {
		let schema = schema_for!(JoinRequest);
		let schema = serde_json::to_string_pretty(&schema)?;

		let path = "join_request.schema.json";
		self.output.write_schema(&path, &schema)
	}
}
