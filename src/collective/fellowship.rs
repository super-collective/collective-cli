#![allow(dead_code)]

use crate::{
	collective::{Collective, EvidenceCategories},
	evidence::EvidenceReport,
	member::{GenericMember},
};
use crate::collective::GenericJoinRequest;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use crate::traits::EnumLike;
use std::borrow::Cow;
use crate::collective::CollectiveId;

pub type FellowshipMember = GenericMember<FellowshipCollective>;
pub type FellowshipEvidenceReport = EvidenceReport<FellowshipCollective>;
pub type FellowshipJoinRequest = GenericJoinRequest<FellowshipCollective>;

/// Something similarly structured as the fellowship, but could have a different rank type.
pub struct FellowshipCollective;

impl Collective for FellowshipCollective {
	const ID: CollectiveId = CollectiveId::Fellowship;
	type Rank = FellowshipRank;
	type EvidenceCategories = FellowshipEvidenceCategory;
	type Member = FellowshipMember;
	const NAME: &'static str = "Polkadot Fellowship";
}

#[repr(u8)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
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

impl crate::traits::Named for FellowshipRank {
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

impl crate::traits::Rank for FellowshipRank {}

impl crate::traits::EnumLike for FellowshipRank {
	fn variants() -> Vec<Self> {
		vec![
			Self::Candidate,
			Self::Humble,
			Self::Proficient,
			Self::Fellow,
			Self::Architect,
			Self::ArchitectAdept,
			Self::GrandArchitect,
			Self::FreeMaster,
			Self::MasterConstant,
			Self::GrandMaster,
		]
	}
}

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
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FellowshipDevelopmentEvidence {
	Sdk,
	Runtime,
	Tooling,
	Other,
}

impl EvidenceCategories for FellowshipEvidenceCategory {}

impl crate::traits::MultiTierNamed for FellowshipEvidenceCategory {
	fn multi_tier_name(&self) -> Vec<Cow<'static, str>> {
		match self {
			Self::Development(dev) =>
				[vec!["Development".into()], dev.multi_tier_name()].concat(),
		}
	}
}

impl EnumLike for FellowshipEvidenceCategory {
	fn variants() -> Vec<Self> {
		FellowshipDevelopmentEvidence::variants()
			.into_iter()
			.map(Self::Development)
			.collect()
	}
}

impl crate::traits::MultiTierNamed for FellowshipDevelopmentEvidence {
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

impl EnumLike for FellowshipDevelopmentEvidence {
	fn variants() -> Vec<Self> {
		vec![Self::Sdk, Self::Runtime, Self::Tooling, Self::Other]
	}
}

#[test]
fn parses_example_file() {
	let file = std::fs::read_to_string("example/example.evidence").unwrap();
	let evidence: FellowshipEvidenceReport = serde_yaml::from_str(&file).unwrap();

	assert_eq!(evidence.name, "Max Power");
}
