use crate::{cmd::plural};
use crate::member::Member;

use std::path::PathBuf;
use crate::fellowship::Rank;
use sailfish::TemplateOnce;
use crate::member::Members;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, clap::Parser)]
pub struct RenderMembersCommand {
	/// The evidence folder.
	#[clap(long, default_value = "members")]
	members: PathBuf,

	#[clap(long, default_value = "members.md")]
	output: PathBuf,
}

impl RenderMembersCommand {
	pub fn run(&self) -> Result<()> {
		self.check_root_folder()?;
		let members = self.parse_files()?;
		log::debug!("Found {} member{} in '{}'", members.len(), plural(members.len()), self.members.display());

		if let Ok(members) = Members::try_from(members) {
			let ctx = crate::template::MembersTemplate { members };
			let rendered = ctx.render_once()?;
			std::fs::write(&self.output, rendered)?;
		} else {
			log::warn!("Members collection is empty");
		}

		Ok(())
	}

	fn check_root_folder(&self) -> Result<()> {
		let root = PathBuf::from(&self.members);

		if !root.exists() {
			return Err(format!("Folder '{}' does not exist", root.display()).into());
		}
		if !root.is_dir() {
			return Err(format!("Folder '{}' is not a directory", root.display()).into());
		}

		Ok(())
	}

	fn parse_files(&self) -> Result<Vec<Member<Rank>>> {
		let mut members = vec![];

		for entry in std::fs::read_dir(&self.members)? {
			let entry = entry?;
			let path = entry.path();

			if path.is_file() && path.extension() == Some("yaml".as_ref()) {
				let file = std::fs::File::open(&path)?;
				let member: Member<Rank> = serde_yaml::from_reader(file)?;

				log::debug!("Parsed member from '{}'", path.display());
				members.push(member);
			}
		}

		Ok(members)
	}
}
