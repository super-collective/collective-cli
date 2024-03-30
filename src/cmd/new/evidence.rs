use crate::{
	cmd::OnOff,
	collective::{
		fellowship::{FellowshipEvidenceCategory, FellowshipEvidenceReport},
		CollectiveId,
	},
	evidence::{Evidence, ReportPeriod, Tasks},
	prompt::Prompt,
	traits::Rank,
};
use crate::traits::EnumLike;
use crate::traits::Query;
use crate::evidence::Wish;
use anyhow::anyhow;
use chrono::{NaiveDate, Weekday};
use inquire::{DateSelect, Select};
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
	pub fn run(&self) -> Result<()> {
		let data = match self.mode {
			GenerationMode::Template => FellowshipEvidenceReport::template().into(),
			GenerationMode::Example => FellowshipEvidenceReport::example().into(),
			GenerationMode::Cli => self.run_prompt()?,
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

	fn run_prompt(&self) -> Result<String> {
		let filled = self.query()?;

		println!("Please fill out the remaining TODOs");
		serde_yaml::to_string(&filled).map_err(Into::into)
	}

	fn query(&self) -> Result<FellowshipEvidenceReport> {
		let mut prompt = Prompt::new(self.cache == OnOff::On)?;

		let name = prompt.query_cached_text::<String>(
			"reporter_legal_name",
			"your legal name",
			Some("You can also use a pseudonym instead"),
		)?;

		let address = prompt.query_cached_text::<String>(
			"reporter_address",
			"your Polkadot address",
			None,
		)?;

		let github = prompt
			.query_cached_text::<String>("reporter_github", "your GitHub handle", None)?
			.replace('@', " ");

		let wish = Wish::<crate::collective::fellowship::FellowshipRank>::query_bare(&mut prompt)?;
		let date = Self::query_date("Creation date of this report")?;
		let report_period_start = Self::query_date("First day that this report covers")?;
		let report_period_end = Self::query_date("Last day that this report covers")?;

		Ok(FellowshipEvidenceReport {
			collective: CollectiveId::Fellowship,
			name,
			address,
			github,
			wish,
			date: date.to_string(),
			period: ReportPeriod {
				start: report_period_start.to_string(),
				end: report_period_end.to_string(),
			},
			evidence: vec![Evidence {
				category: FellowshipEvidenceCategory::variants()[0], //  FIXME
				title: "TODO".into(),
				tasks: vec![Tasks { title: "TODO".into(), links: vec!["TODO".into()] }],
			}],
		})
	}

	fn query_date(title: &str) -> Result<NaiveDate> {
		DateSelect::new(title)
			.with_starting_date(chrono::Utc::now().date_naive())
			.with_week_start(Weekday::Mon)
			.prompt()
			.map_err(Into::into)
	}

	fn query_rank<R: Rank>(title: &str) -> Result<R> {
		let ranks = R::variants();
		let options = ranks
			.iter()
			.enumerate()
			.map(|(i, r)| format!("{i} - {}", r.name()))
			.collect::<Vec<_>>();
		let rank = Select::new(title, options.clone()).prompt()?;
		let index = options.iter().position(|r| r == &rank).unwrap();

		Ok(ranks[index])
	}
}
