use crate::collective::Collective;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use crate::traits::Query;
use crate::collective::{EvidenceCategories, EvidenceCategoriesBaseTrait};
use inquire::Text;
use inquire::Select;
use crate::traits::EnumLike;
use crate::traits::vector_prompt;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportPeriod {
	#[serde(rename = "start_date")]
	pub start: String,
	#[serde(rename = "end_date")]
	pub end: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "intent", content = "rank", rename_all = "lowercase")]
pub enum Wish<Rank> {
	Retain(Rank),
	Promote(Rank),
}

impl<Rank: crate::traits::Rank> Query for Wish<Rank> {
	fn query(_title: Option<&str>, _key: Option<&str>, p: &mut crate::prompt::Prompt) -> anyhow::Result<Self> {
		let options = vec!["retain", "promote"];
		let wish = Select::new("Wish", options.clone()).prompt()?;
		let rank_title = format!("Rank to {}", wish);
		let rank = Rank::query(Some(&rank_title), None, p)?;

		Ok(match wish {
			"retain" => Self::Retain(rank),
			"promote" => Self::Promote(rank),
			_ => unreachable!(),
		})
	}
}

impl<Rank> Wish<Rank> {
	pub fn title(&self) -> &'static str {
		match self {
			Wish::Retain(_) => "Retain",
			Wish::Promote(_) => "Promote",
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tasks {
	pub title: String,
	pub links: Vec<String>,
}

impl Query for Tasks {
	fn query(_title: Option<&str>, _key: Option<&str>, _p: &mut crate::prompt::Prompt) -> anyhow::Result<Self> {
		let title = Text::new("Title a the Task")
			.with_help_message("A piece of evidence consists of multiple tasks.\nEach task should be an atom of contribution.\nFor example a Merge Request or Referendum.").prompt()?;
		let links = vector_prompt("links", || Text::new("Link to Task").prompt())?;

		Ok(Self {
			title,
			links,
		})
	}
}

pub trait EvidenceTrait {
	fn title(&self) -> &str;
	fn tasks(&self) -> &Vec<Tasks>;
	fn category(&self) -> &dyn EvidenceCategoriesBaseTrait;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evidence<EC> {
	pub title: String,
	pub category: EC,
	pub tasks: Vec<Tasks>,
}

impl<EC: EvidenceCategories> EvidenceTrait for Evidence<EC> {
	fn title(&self) -> &str {
		&self.title
	}

	fn tasks(&self) -> &Vec<Tasks> {
		&self.tasks		
	}

	fn category(&self) -> &dyn EvidenceCategoriesBaseTrait {
		&self.category
	}
}

impl<EC: EvidenceCategories> Query for Evidence<EC> {
	fn query(_title: Option<&str>, _key: Option<&str>, p: &mut crate::prompt::Prompt) -> anyhow::Result<Self> {
		let title = Text::new("Title for this piece of evidence").with_help_message("Some example could be 'Fixed lots of bugs', 'Added features' or 'Maintained code' etc.").prompt()?;
		let category = EC::query(Some("Category of the evidence"), None, p)?;
		let tasks = vector_prompt("tasks", || Tasks::query_bare(p))?;

		Ok(Self {
			title,
			category,
			tasks,
		})
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "C::Rank: Deserialize<'de>, C::EvidenceCategories: Deserialize<'de>"))]
#[serde(bound(serialize = "C::Rank: Serialize, C::EvidenceCategories: Serialize"))]
#[serde(rename_all = "snake_case")]
pub struct EvidenceReport<C: Collective> {
	pub name: String,
	pub address: String,
	pub github: String,
	pub wish: Wish<C::Rank>,
	pub collective: crate::collective::CollectiveId,
	#[serde(rename = "report_date")]
	pub date: String,
	#[serde(rename = "report_period")]
	pub period: ReportPeriod,
	#[serde(rename = "evidence")]
	pub evidence: Vec<Evidence<C::EvidenceCategories>>,
}

impl<C: Collective> Query for EvidenceReport<C> {
	fn query(_title: Option<&str>, _key: Option<&str>, prompt: &mut crate::prompt::Prompt) -> anyhow::Result<Self> {
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

		let wish = Wish::<C::Rank>::query_bare(prompt)?;
		let date = prompt.query_date("Creation date of this report")?;
		let report_period_start = prompt.query_date("First day that this report covers")?;
		let report_period_end = prompt.query_date("Last day that this report covers")?;

		Ok(Self {
			collective: C::ID,
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
				category: C::EvidenceCategories::variants()[0], //  FIXME
				title: "TODO".into(),
				tasks: vec![Tasks { title: "TODO".into(), links: vec!["TODO".into()] }],
			}],
		})
	}
}

impl<C: Collective> EvidenceReport<C> {
	pub fn address_link(&self) -> String {
		let shortened = if self.address.len() > 8 {
			format!("{}..", &self.address[..8])
		} else {
			self.address.clone()
		};
		format!(
			"<a target='_blank' href='https://collectives.statescan.io/#/accounts/{}'>{}</a>",
			self.address, shortened
		)
	}

	pub fn github_link(&self) -> String {
		format!("<a target='_blank' href='https://github.com/{}'>{}</a>", self.github, self.github)
	}

	pub fn canonical_name(&self) -> String {
		self.name.to_lowercase().replace(' ', "-")
	}

	pub fn evidence_categories(&self) -> BTreeSet<C::EvidenceCategories> {
		self.evidence.iter().map(|e| e.category).collect()
	}
}

impl<C: Collective> EvidenceReport<C> {
	/// YAML schema in JSON format.
	pub fn schema() -> &'static str {
		include_str!("../schema/evidence_report.json")
	}

	pub fn example() -> &'static str {
		include_str!("../example/example.evidence")
	}

	pub fn template() -> &'static str {
		include_str!("../example/template.evidence")
	}
}


#[test]
fn evidence_encode_works() {
	let evidence = Evidence {
		title: "Fixed a lot of bugs".into(),
		category: crate::collective::fellowship::FellowshipEvidenceCategory::Development(crate::collective::fellowship::FellowshipDevelopmentEvidence::Sdk),
		tasks: vec![Tasks {
			title: "Fixed a bug".into(),
			links: vec!["https://tasty.limo".into()],
		}],
	};
	let encoded = serde_yaml::to_string(&evidence).unwrap();
	assert_eq!(r#"title: Fixed a lot of bugs
category:
  t: development
  c: sdk
tasks:
- title: Fixed a bug
  links:
  - https://tasty.limo
"#, encoded);
}
