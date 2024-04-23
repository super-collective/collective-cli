// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

mod aggregate;
mod report;
mod traits;

pub use aggregate::*;
pub use report::*;
pub use traits::*;

use crate::types::prelude::*;
use inquire::Text;
#[cfg(test)]
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

/// A specific piece of evidence to help underpin an argument.
#[derive(Debug, Serialize, Deserialize, Clone, schemars::JsonSchema)]
#[schemars(bound = "EC: schemars::JsonSchema", rename = "Evidence")]
pub struct GenericEvidence<EC> {
	/// Title of this piece of evidence.
	pub title: String,
	/// Topic category of the evidence.
	pub category: EC,
	/// Tasks that are part of this evidence.
	pub tasks: Vec<Tasks>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportPeriod {
	/// First inclusive date that this report covers.
	#[serde(rename = "start_date")]
	pub start: String,
	/// Last inclusive date that this report covers.
	#[serde(rename = "end_date")]
	pub end: String,
}

/// A task that was either solely completed by the reporter or in collaboration with others.
#[derive(Debug, Serialize, Deserialize, Clone, schemars::JsonSchema)]
pub struct Tasks {
	/// Title of the task.
	pub title: String,
	/// Links to the completed work.
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

impl<EC: EvidenceCategories> EvidenceTrait for GenericEvidence<EC> {
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

impl<EC: EvidenceCategories> Query for GenericEvidence<EC> {
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
	let evidence = GenericEvidence {
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
