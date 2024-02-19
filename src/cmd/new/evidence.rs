use crate::{
	cmd::OutputArgs,
	evidence::{Collective, Evidence, ReportPeriod, Tasks, WishUntyped},
	fellowship::FellowshipReport,
	prompt::Prompt,
	traits::Rank,
};

use chrono::{NaiveDate, Weekday};
use inquire::{DateSelect, Select};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, clap::Parser)]
pub struct NewEvidenceCommand {
	#[clap(index = 1, default_value = "cli")]
	mode: GenerationMode,

	#[clap(flatten)]
	output: OutputArgs,
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
			GenerationMode::Template => FellowshipReport::template().into(),
			GenerationMode::Example => FellowshipReport::example().into(),
			GenerationMode::Cli => self.run_prompt()?,
		};

		self.output.write_evidence(&data)
	}

	fn run_prompt(&self) -> Result<String> {
		let filled = self.query()?;

		println!("Please fill out the remaining TODOs");
		serde_yaml::to_string(&filled).map_err(Into::into)
	}

	fn query(&self) -> Result<FellowshipReport> {
		let mut prompt = Prompt::new()?;

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

		let wish = Self::query_wish::<crate::fellowship::Rank>()?;
		let date = Self::query_date("Creation date of this report")?;
		let report_period_start = Self::query_date("First day that this report covers")?;
		let report_period_end = Self::query_date("Last day that this report covers")?;

		Ok(FellowshipReport {
			collective: Collective::Fellowship,
			name,
			address,
			github,
			wish,
			date: date.to_string(),
			period: ReportPeriod {
				start: report_period_start.to_string(),
				end: report_period_end.to_string(),
			},
			categories: vec![],
			evidence: vec![Evidence {
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

	fn query_wish<R: Rank>() -> Result<WishUntyped<R>> {
		let wish =
			Select::new("Your wish regarding your rank", vec!["retain", "promote"]).prompt()?;

		match wish {
			"retain" => {
				let rank = Self::query_rank::<R>(
					"Your desired rank that you would like to be promoted to",
				)?;

				Ok(WishUntyped { retain: Some(rank), promote: None })
			},
			"prompt" => {
				let rank = Self::query_rank::<R>(
					"Your desired rank that you would like to be promoted to",
				)?;

				Ok(WishUntyped { promote: Some(rank), retain: None })
			},
			_ => unreachable!(),
		}
	}

	fn query_rank<R: Rank>(title: &str) -> Result<R> {
		let ranks = R::possible_values();
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
