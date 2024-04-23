// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

mod evidence;

#[derive(Debug, clap::Parser)]
pub struct IndexCommand {
	#[clap(subcommand)]
	subcommand: IndexSubCommand,
}

#[derive(Debug, clap::Subcommand)]
enum IndexSubCommand {
	/// Create some evidence Index.
	Evidence(evidence::IndexEvidenceCommand),
}

impl IndexCommand {
	pub fn run(&self) -> anyhow::Result<()> {
		match &self.subcommand {
			IndexSubCommand::Evidence(c) => c.run(),
		}
	}
}
