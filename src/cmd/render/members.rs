use crate::cmd::plural;

use crate::{
	config::GlobalConfig,
	member::{Member, Members},
};
use anyhow::Context;
use anyhow::bail;
use sailfish::TemplateOnce;
use std::path::PathBuf;

type Result<T> = anyhow::Result<T>;

#[derive(Debug, clap::Parser)]
pub struct RenderMembersCommand {
	#[clap(long, default_value = "members.md")]
	output: PathBuf,
}

impl RenderMembersCommand {
	pub fn run(&self, g: &GlobalConfig) -> Result<()> {
		self.check_root_folder(g).context("checking root folder")?;
		let members_dir = &g.members_dir;
		let members = self.parse_files(g)?;
		log::debug!(
			"Found {} member{} in '{}'",
			members.len(),
			plural(members.len()),
			members_dir.display()
		);

		if let Ok(members) = Members::try_from(members) {
			let ctx = crate::template::MembersTemplate { members };
			let rendered = ctx.render_once()?;
			std::fs::write(&self.output, rendered)?;
			log::info!("Rendered members to '{}'", self.output.display());
		} else {
			log::warn!("Members collection is empty");
		}

		Ok(())
	}

	fn check_root_folder(&self, g: &GlobalConfig) -> Result<()> {
		let root = PathBuf::from(&g.members_dir);

		if !root.exists() {
			bail!("Folder '{}' does not exist", root.display());
		}
		if !root.is_dir() {
			bail!("Folder '{}' is not a directory", root.display());
		}

		Ok(())
	}

	fn parse_files(&self, g: &GlobalConfig) -> Result<Vec<Member>> {
		let mut members: Vec<Member> = vec![];

		for entry in std::fs::read_dir(&g.members_dir)? {
			let entry = entry?;
			let path = entry.path();

			if path.is_file() && path.extension() == Some("yaml".as_ref()) {
				let file = std::fs::File::open(&path)?;
				let member: Member = serde_yaml::from_reader(file)?;

				log::debug!("Parsed member from '{}'", path.display());
				members.push(member);
			}
		}

		Ok(members)
	}
}
