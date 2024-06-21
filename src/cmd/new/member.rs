// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{
	config::{GlobalArgs, GlobalConfig},
	types::prelude::*,
};
use anyhow::{bail, Context};
use std::path::PathBuf;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, clap::Parser)]
pub struct NewMemberCommand {
	#[clap(flatten)]
	global: GlobalArgs,

	/// The join request of the member that should be accepted.
	#[clap(long, short = 'r')]
	join_request: PathBuf,
}

impl NewMemberCommand {
	pub fn run(&self, g: &GlobalConfig) -> Result<()> {
		let path = &self.join_request;
		if !path.exists() {
			bail!("evidence file does not exist: {}", path.display());
		}
		let file = std::fs::read_to_string(path.as_path()).context("reading evidence file")?;
		let request: JoinRequest =
			serde_yaml::from_str(&file).with_context(|| format!("parsing {:?}", path))?;
		
		let member = request.member();
		let mut target_path = g.members_dir.join(member.github());
		target_path.set_extension("yml");
		if target_path.exists() {
			bail!("member already exists: {}", target_path.display());
		}
		
		let member = match &request {
			JoinRequest::Fellowship(r) => Member::Fellowship(r.member.clone()),
			JoinRequest::Potoc(r) => Member::Potoc(r.member.clone()),
		};

		let encoded = serde_yaml::to_string(&member).context("encoding member")?;
		std::fs::write(&target_path, encoded).context("writing member file")?;
		println!("🎉 Accepted member '{}' into '{}'.", member.name(), target_path.display());

		Ok(())
	}
}
