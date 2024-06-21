// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

mod evidence;
mod join_request;
mod member;

use crate::config::GlobalConfig;

#[derive(Debug, clap::Parser)]
pub struct NewCommand {
	#[clap(subcommand)]
	subcommand: NewSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum NewSubCommand {
	/// Create a new evidence report.
	Evidence(evidence::NewEvidenceCommand),
	// Create a new member request.
	JoinRequest(join_request::NewJoinRequestCommand),
	// Create a new member.
	Member(member::NewMemberCommand),
}

impl NewCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		match &self.subcommand {
			NewSubCommand::Evidence(c) => c.run(g),
			NewSubCommand::JoinRequest(c) => c.run(g),
			NewSubCommand::Member(c) => c.run(g),
		}
	}
}
