use glob::glob;
use std::path::PathBuf;
use crate::config::GlobalConfig;
use valico::{json_schema, json_schema::schema::ScopedSchema};
use crate::collective::fellowship::FellowshipEvidenceReport;
use crate::member::JoinRequest;

type Result<T> = anyhow::Result<T>;

#[derive(Debug, clap::Parser)]
pub struct CheckJoinRequestCommand {
	/// Explicit list of paths to evidence reports.
	#[clap(long)]
	files: Option<Vec<PathBuf>>,
}

impl CheckJoinRequestCommand {
	pub fn run(&self, g: &GlobalConfig) -> Result<()> {
		let paths = self.relevant_files(g)?;

		for path in paths.iter() {
			let data = std::fs::read_to_string(path.as_path())?;

			// Check that we can decode it.
			let _: JoinRequest = serde_yaml::from_str(&data)?;
		}

		println!("Validated {} join request{}.", paths.len(), crate::cmd::plural(paths.len()));

		Ok(())
	}

	fn relevant_files(&self, g: &GlobalConfig) -> Result<Vec<PathBuf>> {
		if let Some(files) = &self.files {
			Ok(files.clone())
		} else {
			let pattern = format!("{}/*.yaml", g.join_requests_dir.display());
			let mut files = vec![];

			for entry in glob(&pattern)? {
				files.push(entry?);
			}

			Ok(files)
		}
	}
}