use crate::{prompt::Prompt, types::prelude::*, using_collective};
use serde::{Deserialize, Serialize};

use crate::collective::{fellowship::FellowshipEvidenceReport, potoc::PotocEvidenceReport};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "collective")]
pub enum EvidenceReport {
	Fellowship(FellowshipEvidenceReport),
	Potoc(PotocEvidenceReport),
}

impl EvidenceReport {
	pub fn collective(&self) -> CollectiveId {
		match self {
			Self::Fellowship(_) => CollectiveId::Fellowship,
			Self::Potoc(_) => CollectiveId::Potoc,
		}
	}

	pub fn wish(&self) -> &dyn WishTrait {
		using_collective!(self, request, { &request.wish })
	}

	pub fn member(&self) -> &dyn MemberTrait {
		using_collective!(self, request, { &request.member })
	}

	pub fn date(&self) -> &str {
		using_collective!(self, request, { &request.date })
	}

	pub fn period(&self) -> &ReportPeriod {
		using_collective!(self, request, { &request.period })
	}

	pub fn evidence(&self) -> Vec<&dyn EvidenceTrait> {
		using_collective!(self, request, {
			request.evidence.iter().map(|e| e as &dyn EvidenceTrait).collect()
		})
	}

	pub fn evidence_categories(&self) -> Vec<&dyn EvidenceCategoriesBaseTrait> {
		using_collective!(self, request, {
			request.evidence.iter().map(|e| e.category()).collect()
		})
	}
}

impl Query for EvidenceReport {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		prompt: &mut Prompt,
	) -> anyhow::Result<Self> {
		let id = CollectiveId::query(Some("Collective to join"), None, prompt)?;
		Self::query_with_id(&id, prompt)
	}
}

impl EvidenceReport {
	pub fn query_with_id(id: &CollectiveId, prompt: &mut Prompt) -> anyhow::Result<Self> {
		match id {
			CollectiveId::Fellowship =>
				Ok(Self::Fellowship(FellowshipEvidenceReport::query_bare(prompt)?)),
			CollectiveId::Potoc => Ok(Self::Potoc(PotocEvidenceReport::query_bare(prompt)?)),
		}
	}
}
