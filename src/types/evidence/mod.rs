mod report;
mod traits;

pub use report::*;
pub use traits::*;

use crate::types::prelude::*;
use inquire::Text;
#[cfg(test)]
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evidence<EC> {
	pub title: String,
	pub category: EC,
	pub tasks: Vec<Tasks>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportPeriod {
	#[serde(rename = "start_date")]
	pub start: String,
	#[serde(rename = "end_date")]
	pub end: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tasks {
	pub title: String,
	pub links: Vec<String>,
}

impl Query for Tasks {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		_p: &mut crate::prompt::Prompt,
	) -> anyhow::Result<Self> {
		let title = Text::new("Title a the Task")
			.with_help_message("A piece of evidence consists of multiple tasks.\nEach task should be an atom of contribution.\nFor example a Merge Request or Referendum.").prompt()?;
		let links = vector_prompt("links", || Text::new("Link to Task").prompt())?;

		Ok(Self { title, links })
	}
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
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		p: &mut crate::prompt::Prompt,
	) -> anyhow::Result<Self> {
		let title = Text::new("Title for this piece of evidence").with_help_message("Some example could be 'Fixed lots of bugs', 'Added features' or 'Maintained code' etc.").prompt()?;
		let category = EC::query(Some("Category of the evidence"), None, p)?;
		let tasks = vector_prompt("tasks", || Tasks::query_bare(p))?;

		Ok(Self { title, category, tasks })
	}
}

#[test]
fn evidence_encode_works() {
	let evidence = Evidence {
		title: "Fixed a lot of bugs".into(),
		category: crate::collective::fellowship::FellowshipEvidenceCategory::Development(
			crate::collective::fellowship::FellowshipDevelopmentEvidence::Sdk,
		),
		tasks: vec![Tasks {
			title: "Fixed a bug".into(),
			links: vec!["https://tasty.limo".into()],
		}],
	};
	let encoded = serde_yaml::to_string(&evidence).unwrap();
	assert_eq!(
		r#"title: Fixed a lot of bugs
category:
  t: development
  c: sdk
tasks:
- title: Fixed a bug
  links:
  - https://tasty.limo
"#,
		encoded
	);
}
