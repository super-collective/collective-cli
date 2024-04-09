use crate::{
	collective::{Collective, CollectiveId},
	prompt::Prompt,
	traits::{Decode, Encode, Named, Query},
	using_collective,
};
use crate::traits::RankBaseTrait;
use core::fmt::Debug;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::collective::fellowship::FellowshipJoinRequest;
use crate::collective::potoc::PotocJoinRequest;
use crate::evidence::EvidenceTrait;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "collective")]
pub enum Member {
	Fellowship(<crate::collective::fellowship::FellowshipCollective as Collective>::Member),
	Potoc(<crate::collective::potoc::PotocCollective as Collective>::Member),
}

pub trait MemberTrait: Named {
	fn github(&self) -> &str;
	fn address(&self) -> &str;
	fn rank(&self) -> &dyn RankBaseTrait;
}

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
		using_collective!(self, request, { request.evidence.iter().map(|e| e as &dyn EvidenceTrait).collect() })
	}

	pub fn evidence_categories(&self) -> Vec<&dyn crate::collective::EvidenceCategoriesBaseTrait> {
		using_collective!(self, request, { request.evidence.iter().map(|e| e.category()).collect() })
	}
}

impl Query for JoinRequest {
	fn query(_title: Option<&str>, _key: Option<&str>, prompt: &mut Prompt) -> anyhow::Result<Self> {
		Ok(match CollectiveId::query(Some("Collective to join"), None, prompt)? {
			CollectiveId::Fellowship => Self::Fellowship(FellowshipJoinRequest::query_bare(prompt)?),
			CollectiveId::Potoc => Self::Potoc(PotocJoinRequest::query_bare(prompt)?),
		})
	}
}

// TODO split into member and ranked-member
#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug(bound = "C::Rank: Debug"))]
#[derivative(Clone(bound = "C::Rank: Clone"))]
pub struct GenericMember<C: Collective> {
	pub name: String,
	pub address: String,
	pub github: String,
	pub rank: C::Rank,
}

impl<C: Collective> MemberTrait for GenericMember<C> {
	fn github(&self) -> &str {
		&self.github
	}

	fn address(&self) -> &str {
		&self.address
	}

	fn rank(&self) -> &dyn RankBaseTrait {
		&self.rank
	}
}

impl MemberTrait for Member {
	fn github(&self) -> &str {
		using_collective!(self, member, { member.github() })
	}

	fn address(&self) -> &str {
		using_collective!(self, member, { member.address() })
	}

	fn rank(&self) -> &dyn RankBaseTrait {
		using_collective!(self, member, { member.rank() })
	}
}

impl Member {
	pub fn collective(&self) -> CollectiveId {
		match self {
			Self::Fellowship(_) => CollectiveId::Fellowship,
			Self::Potoc(_) => CollectiveId::Potoc,
		}
	}
}

impl<C: Collective> Encode for GenericMember<C> {
	fn to_yaml(&self) -> serde_yaml::Value {
		serde_yaml::to_value(self).unwrap()
	}
}

impl<C: Collective> Decode for GenericMember<C> {
	fn from_yaml(value: serde_yaml::Value) -> anyhow::Result<Self> {
		serde_yaml::from_value(value).map_err(Into::into)
	}
}

impl<C: Collective> Named for GenericMember<C> {
	fn name(&self) -> Cow<'static, str> {
		self.name.clone().into()
	}
}

impl Named for Member {
	fn name(&self) -> Cow<'static, str> {
		match self {
			Self::Fellowship(member) => member.name(),
			Self::Potoc(member) => member.name(),
		}
	}
}

/// Non-empty collection of [Member]s.
#[derive(Debug)]
pub struct Members {
	pub members: Vec<Member>,
	pub collective: CollectiveId,
}

impl<C: Collective> Query for GenericMember<C> {
	fn query(title: Option<&str>, _key: Option<&str>, prompt: &mut Prompt) -> anyhow::Result<Self> {
		let name = prompt.query_cached_text::<String>(
			"reporter_legal_name",
			"legal name or pseudonym",
			None,
		)?;

		let address = prompt.query_cached_text::<String>(
			"reporter_address",
			"your Polkadot address",
			None,
		)?;

		let github = prompt
			.query_cached_text::<String>("reporter_github", "your GitHub handle", None)?
			.replace('@', " ");
		
		let rank_title = if let Some(title) = title {
			format!("Rank to {title}")
		} else {
			"Rank".into()
		};
		let rank = C::Rank::query(Some(&rank_title), None, prompt)?;

		Ok(Self {
			name,
			address,
			github,
			rank
		})
	}
}

impl TryFrom<Vec<Member>> for Members {
	type Error = &'static str;

	fn try_from(members: Vec<Member>) -> Result<Self, Self::Error> {
		if members.is_empty() {
			Err("Members collection cannot be empty")
		} else {
			Ok(Self { collective: members[0].collective(), members })
		}
	}
}