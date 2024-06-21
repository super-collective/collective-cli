// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

mod evidence;
mod join_request;
mod member;

use crate::config::GlobalConfig;

#[derive(Debug, clap::Parser)]
pub struct CheckCommand {
	#[clap(subcommand)]
	subcommand: CheckSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum CheckSubCommand {
	/// Check one or more evidence reports for formatting errors.
	Evidence(evidence::CheckEvidenceCommand),
	/// Check a join-request for formatting errors.
	JoinRequest(join_request::CheckJoinRequestCommand),
	/// Check a member for formatting errors.
	Member(member::CheckMemberCommand),
}

impl CheckCommand {
	pub fn run(&self, g: &GlobalConfig) -> anyhow::Result<()> {
		match &self.subcommand {
			CheckSubCommand::Evidence(c) => c.run(g),
			CheckSubCommand::JoinRequest(c) => c.run(g),
			CheckSubCommand::Member(c) => c.run(g),
		}
	}
}
