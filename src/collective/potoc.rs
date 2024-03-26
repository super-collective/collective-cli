#![allow(dead_code)]

use serde_repr::*;
use crate::collective::Collective;
use crate::evidence::EvidenceReport;
use crate::member::Member;

pub type PotocMember = Member<PotocCollective>;
pub type PotocEvidenceReport = EvidenceReport<PotocCollective>;

/// Something similarly structured as the Potoc, but could have a different rank type.
pub struct PotocCollective;

impl Collective for PotocCollective {
	type Rank = PotocRank;
	const NAME: &'static str = "Polkadot Tooling Collective";
}

#[repr(u8)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
pub enum PotocRank {
	Candidate = 0,
	Member = 1,
}

impl crate::traits::Named for PotocRank {
	fn name(&self) -> &'static str {
		match self {
			Self::Candidate => "Candidate",
			Self::Member => "Member",
		}
	}
}

impl crate::traits::Rank for PotocRank {
	fn possible_values() -> Vec<Self> {
		vec![
			Self::Candidate,
			Self::Member,
		]
	}
}

impl From<PotocRank> for u32 {
	fn from(rank: PotocRank) -> u32 {
		rank as u32
	}
}

#[test]
fn parses_example_file() {
	let file = std::fs::read_to_string("example/example.evidence").unwrap();
	let evidence: PotocEvidenceReport = serde_yaml::from_str(&file).unwrap();

	assert_eq!(evidence.name, "Max Power");
}
