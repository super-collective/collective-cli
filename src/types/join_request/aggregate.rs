use serde::{Deserialize, Serialize};

use crate::{
	collective::{fellowship::FellowshipJoinRequest, potoc::PotocJoinRequest, CollectiveId},
	prompt::Prompt,
	traits::Query,
	types::{
		evidence::EvidenceCategoriesBaseTrait,
		prelude::{EvidenceTrait, MemberTrait},
	},
	using_collective,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "collective")]
pub enum JoinRequest {
	Fellowship(FellowshipJoinRequest),
	Potoc(PotocJoinRequest),
}

impl JoinRequest {
	pub fn collective(&self) -> CollectiveId {
		match self {
			Self::Fellowship(_) => CollectiveId::Fellowship,
			Self::Potoc(_) => CollectiveId::Potoc,
		}
	}

	pub fn member(&self) -> &dyn MemberTrait {
		using_collective!(self, request, { &request.member })
	}

	pub fn date(&self) -> &str {
		using_collective!(self, request, { &request.date })
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

	pub fn about(&self) -> &str {
		using_collective!(self, request, { &request.about })
	}

	pub fn motivation(&self) -> &str {
		using_collective!(self, request, { &request.motivation })
	}
}

impl Query for JoinRequest {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		prompt: &mut Prompt,
	) -> anyhow::Result<Self> {
		let id = CollectiveId::query(Some("Collective to join"), None, prompt)?;
		Self::query_with_id(&id, prompt)
	}
}

impl JoinRequest {
	pub fn query_with_id(id: &CollectiveId, prompt: &mut Prompt) -> anyhow::Result<Self> {
		match id {
			CollectiveId::Fellowship =>
				Ok(Self::Fellowship(FellowshipJoinRequest::query_bare(prompt)?)),
			CollectiveId::Potoc => Ok(Self::Potoc(PotocJoinRequest::query_bare(prompt)?)),
		}
	}
}
