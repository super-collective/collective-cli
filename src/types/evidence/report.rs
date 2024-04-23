// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::{prelude::*, traits::Query};
use core::fmt::Debug;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug(bound = "C::Member: Debug, C::EvidenceCategories: Debug"))]
#[serde(bound(deserialize = "C::Rank: Deserialize<'de>, C::EvidenceCategories: Deserialize<'de>"))]
#[serde(bound(serialize = "C::Rank: Serialize, C::EvidenceCategories: Serialize"))]
#[serde(rename_all = "snake_case")]
pub struct GenericEvidenceReport<C: CollectiveTrait> {
	pub member: C::Member,
	pub wish: Wish<C::Rank>,
	#[serde(skip, default = "GenericEvidenceReport::<C>::id")]
	pub collective: CollectiveId,
	#[serde(rename = "report_date")]
	pub date: String,
	#[serde(rename = "report_period")]
	pub period: ReportPeriod,
	#[serde(rename = "evidence")]
	pub evidence: Vec<GenericEvidence<C::EvidenceCategories>>,
}

impl<C: CollectiveTrait> GenericEvidenceReport<C> {
	fn id() -> CollectiveId {
		C::ID
	}
}

impl<C: CollectiveTrait> Query for GenericEvidenceReport<C> {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		p: &mut crate::prompt::Prompt,
	) -> anyhow::Result<Self> {
		let member = C::Member::query(Some("that you currently have"), None, p)?;

		let wish = Wish::<C::Rank>::query_bare(p)?;
		let date = p.query_date("Creation date of this report")?;
		let report_period_start = p.query_date("First day that this report covers")?;
		let report_period_end = p.query_date("Last day that this report covers")?;

		Ok(Self {
			collective: C::ID,
			member,
			wish,
			date: date.to_string(),
			period: ReportPeriod {
				start: report_period_start.to_string(),
				end: report_period_end.to_string(),
			},
			evidence: vec![GenericEvidence {
				category: C::EvidenceCategories::variants()[0], //  FIXME
				title: "TODO".into(),
				tasks: vec![Tasks { title: "TODO".into(), links: vec!["TODO".into()] }],
			}],
		})
	}
}

impl<C: CollectiveTrait> GenericEvidenceReport<C> {
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
