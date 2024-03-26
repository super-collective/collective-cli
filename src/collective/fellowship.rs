#![allow(dead_code)]

use serde_repr::*;
use crate::collective::Collective;
use crate::evidence::EvidenceReport;
use crate::member::Member;

pub type FellowshipMember = Member<FellowshipCollective>;
pub type FellowshipEvidenceReport = EvidenceReport<FellowshipCollective>;

/// Something similarly structured as the fellowship, but could have a different rank type.
pub struct FellowshipCollective;

impl Collective for FellowshipCollective {
	type Rank = FellowshipRank;
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
	fn name(&self) -> &'static str {
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
	}
}

impl crate::traits::Rank for FellowshipRank {
	fn possible_values() -> Vec<Self> {
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

#[test]
fn parses_example_file() {
	let file = std::fs::read_to_string("example/example.evidence").unwrap();
	let evidence: FellowshipEvidenceReport = serde_yaml::from_str(&file).unwrap();

	assert_eq!(evidence.name, "Max Power");
}
