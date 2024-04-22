use crate::types::prelude::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::borrow::Cow;
use strum::IntoEnumIterator;

/// The Polkadot Fellowship Collective.
pub struct FellowshipCollective;

impl CollectiveTrait for FellowshipCollective {
	const NAME: &'static str = "Fellowship";
	const NICKNAME: &'static str = "The Fellowship";
	const ID: CollectiveId = CollectiveId::Fellowship;
	const MANIFESTO: &'static str = "https://github.com/polkadot-fellows/manifesto";

	type Rank = FellowshipRank;
	type EvidenceCategories = FellowshipEvidenceCategory;
	type Member = FellowshipMember;
}

pub type FellowshipMember = GenericMember<FellowshipCollective>;
pub type FellowshipEvidenceReport = GenericEvidenceReport<FellowshipCollective>;
pub type FellowshipJoinRequest = GenericJoinRequest<FellowshipCollective>;

#[repr(u8)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone, strum::EnumIter)]
pub enum FellowshipRank {
	Candidate = 0,
	Humble = 1,
	Proficient = 2,
	Fellow = 3,
	Architect = 4,
	ArchitectAdept = 5,
	GrandArchitect = 6,
	FreeMaster = 7,
	MasterConstant = 8,
	GrandMaster = 9,
}

impl Named for FellowshipRank {
	fn name(&self) -> Cow<'static, str> {
		match self {
			Self::Candidate => "Candidate",
			Self::Humble => "Humble",
			Self::Proficient => "Proficient",
			Self::Fellow => "Fellow",
			Self::Architect => "Architect",
			Self::ArchitectAdept => "Architect Adept",
			Self::GrandArchitect => "Grand Architect",
			Self::FreeMaster => "Free Master",
			Self::MasterConstant => "Master Constant",
			Self::GrandMaster => "Grand Master",
		}
		.into()
	}
}

impl RankBaseTrait for FellowshipRank {}

impl From<FellowshipRank> for u32 {
	fn from(rank: FellowshipRank) -> u32 {
		rank as u32
	}
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "t", content = "c")]
pub enum FellowshipEvidenceCategory {
	Development(FellowshipDevelopmentEvidence),
	Spec(FellowshipSpecEvidence),
}

#[derive(
	Debug, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum FellowshipDevelopmentEvidence {
	Sdk,
	Runtime,
	Tooling,
	Other,
}

impl EvidenceCategoriesBaseTrait for FellowshipEvidenceCategory {}

impl MultiTierNamed for FellowshipEvidenceCategory {
	fn multi_tier_name(&self) -> Vec<Cow<'static, str>> {
		match self {
			Self::Development(dev) => [vec!["Development".into()], dev.multi_tier_name()].concat(),
			Self::Spec(spec) => [vec!["Spec".into()], spec.multi_tier_name()].concat(),
		}
	}
}

impl EnumLike for FellowshipEvidenceCategory {
	fn variants() -> Vec<Self> {
		[
			FellowshipDevelopmentEvidence::iter().map(Self::Development).collect::<Vec<_>>(),
			FellowshipSpecEvidence::iter().map(Self::Spec).collect::<Vec<_>>(),
		]
		.concat()
	}
}

impl MultiTierNamed for FellowshipDevelopmentEvidence {
	fn multi_tier_name(&self) -> Vec<Cow<'static, str>> {
		vec![match self {
			Self::Sdk => "SDK",
			Self::Runtime => "Runtime",
			Self::Tooling => "Tooling",
			Self::Other => "Other",
		}
		.into()]
	}
}

#[derive(
	Debug, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum FellowshipSpecEvidence {
	Rfc,
	Docs,
}

impl MultiTierNamed for FellowshipSpecEvidence {
	fn multi_tier_name(&self) -> Vec<Cow<'static, str>> {
		vec![match self {
			Self::Rfc => "RFC",
			Self::Docs => "Docs",
		}
		.into()]
	}
}

#[test]
fn encodes_evidence_category() {
	let category = FellowshipEvidenceCategory::Development(FellowshipDevelopmentEvidence::Sdk);
	let encoded = serde_yaml::to_string(&category).unwrap();
	assert_eq!(encoded, "t: development\nc: sdk\n");
}

#[test]
fn parses_example_file() {
	let file = std::fs::read_to_string("example/example.evidence").unwrap();
	let evidence: FellowshipEvidenceReport = serde_yaml::from_str(&file).unwrap();

	assert_eq!(evidence.member.name(), "Max Power");
}
