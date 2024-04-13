use crate::types::{prelude::*, traits::Query};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "C::Rank: Deserialize<'de>, C::EvidenceCategories: Deserialize<'de>"))]
#[serde(bound(serialize = "C::Rank: Serialize, C::EvidenceCategories: Serialize"))]
#[serde(rename_all = "snake_case")]
pub struct EvidenceReport<C: Collective> {
	pub name: String,
	pub address: String,
	pub github: String,
	pub wish: Wish<C::Rank>,
	pub collective: CollectiveId,
	#[serde(rename = "report_date")]
	pub date: String,
	#[serde(rename = "report_period")]
	pub period: ReportPeriod,
	#[serde(rename = "evidence")]
	pub evidence: Vec<Evidence<C::EvidenceCategories>>,
}

impl<C: Collective> Query for EvidenceReport<C> {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		prompt: &mut crate::prompt::Prompt,
	) -> anyhow::Result<Self> {
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
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema/evidence_report.json"))
	}

	pub fn example() -> &'static str {
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/example/example.evidence"))
	}

	pub fn template() -> &'static str {
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/example/template.evidence"))
	}
}
