#![allow(dead_code)]

use crate::{
	collective::{Collective, EvidenceCategories, EvidenceCategoriesBaseTrait},
	evidence::EvidenceReport,
	member::{GenericMember},
};
use crate::collective::GenericJoinRequest;
use crate::traits::EnumLike;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::borrow::Cow;
use crate::collective::CollectiveId;

pub type PotocMember = GenericMember<PotocCollective>;
pub type PotocEvidenceReport = EvidenceReport<PotocCollective>;
pub type PotocJoinRequest = GenericJoinRequest<PotocCollective>;

/// Something similarly structured as the Potoc, but could have a different rank type.
pub struct PotocCollective;

impl Collective for PotocCollective {
	const ID: CollectiveId = CollectiveId::Fellowship;
	type Rank = PotocRank;
	type EvidenceCategories = PotocEvidenceCategory;
	type Member = PotocMember;
	const NAME: &'static str = "Tooling Collective";
	const NICKNAME: &'static str = "PoToC";
}

#[repr(u8)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
pub enum PotocRank {
	Candidate = 0,
	Member = 1,
}

impl crate::traits::Named for PotocRank {
	fn name(&self) -> Cow<'static, str> {
		match self {
			Self::Candidate => "Candidate",
			Self::Member => "Member",
		}
		.into()
	}
}

impl crate::traits::RankBaseTrait for PotocRank {}
impl crate::traits::Rank for PotocRank {}

impl crate::traits::EnumLike for PotocRank {
	fn variants() -> Vec<Self> {
		vec![Self::Candidate, Self::Member]
	}
}

impl From<PotocRank> for u32 {
	fn from(rank: PotocRank) -> u32 {
		rank as u32
	}
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PotocEvidenceCategory {
	CoreTooling,
	DAppTooling,
}

impl EvidenceCategoriesBaseTrait for PotocEvidenceCategory {}
impl EvidenceCategories for PotocEvidenceCategory {}

impl crate::traits::MultiTierNamed for PotocEvidenceCategory {
	fn multi_tier_name(&self) -> Vec<Cow<'static, str>> {
		vec![match self {
			Self::CoreTooling => "Core Tooling",
			Self::DAppTooling => "dApp Tooling",
		}
		.into()]
	}
}

impl EnumLike for PotocEvidenceCategory {
	fn variants() -> Vec<Self> {
		vec![Self::CoreTooling, Self::DAppTooling]
	}
}

//#[test]
fn parses_example_file() {
	let file = std::fs::read_to_string("example/example.evidence").unwrap();
	let evidence: PotocEvidenceReport = serde_yaml::from_str(&file).unwrap();

	assert_eq!(evidence.name, "Max Power");
}
