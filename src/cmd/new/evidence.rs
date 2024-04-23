// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{
	cmd::OnOff, collective::fellowship::FellowshipEvidenceReport, config::GlobalConfig,
	prompt::Prompt, types::prelude::*,
};
use anyhow::anyhow;
use std::path::PathBuf;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, clap::Parser)]
pub struct NewEvidenceCommand {
	#[clap(index = 1, default_value = "cli")]
	mode: GenerationMode,

	/// The evidence folder.
	#[clap(long, default_value = "evidence")]
	evidence: PathBuf,

	/// Whether to cache the answers.
	#[clap(long, default_value = "on")]
	cache: OnOff,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum GenerationMode {
	/// A template evidence.
	Template,
	/// An example evidence.
	Example,
	/// CLI will prompt for the evidence.
	Cli,
}

impl NewEvidenceCommand {
	pub fn run(&self, g: &GlobalConfig) -> Result<()> {
		let data = match self.mode {
			GenerationMode::Template => FellowshipEvidenceReport::template().into(),
			GenerationMode::Example => FellowshipEvidenceReport::example().into(),
			GenerationMode::Cli => self.run_prompt(g)?,
		};

		std::fs::create_dir_all(&self.evidence)?;
		let path = self.find_good_path()?;
		std::fs::write(&path, data)?;
		println!("🎉 Wrote partial evidence report to '{}'.", path.display());

		Ok(())
	}

	fn find_good_path(&self) -> Result<PathBuf> {
		for i in 0..1000 {
			// lol shit code
			let path = self.evidence.join(i.to_string()).with_extension("evidence");

			if !path.exists() {
				return Ok(path);
			}
		}

		Err(anyhow!("Could not find a good path. Please use `--evidence` to specify an empty."))
	}

	fn run_prompt(&self, g: &GlobalConfig) -> Result<String> {
		let filled = self.query(g)?;

		println!("Please fill out the remaining TODOs");
		serde_yaml::to_string(&filled).map_err(Into::into)
	}

	fn query(&self, g: &GlobalConfig) -> Result<EvidenceReport> {
		let mut prompt = Prompt::new(self.cache == OnOff::On)?;
		EvidenceReport::query_with_id(&g.collective, &mut prompt)
	}

	/*fn query_date(title: &str) -> Result<NaiveDate> {
		DateSelect::new(title)
			.with_starting_date(chrono::Utc::now().date_naive())
			.with_week_start(Weekday::Mon)
			.prompt()
			.map_err(Into::into)
	}*/
}
