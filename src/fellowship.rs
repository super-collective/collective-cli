use serde_repr::*;

pub type FellowshipReport = crate::evidence::EvidenceReport<Rank>;

#[repr(u8)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
pub enum Rank {
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

impl crate::traits::Rank for Rank {
	fn name(&self) -> &'static str {
		match self {
			Rank::Candidate => "Candidate",
			Rank::Humble => "Humble",
			Rank::Proficient => "Proficient",
			Rank::Fellow => "Fellow",
			Rank::Architect => "Architect",
			Rank::ArchitectAdept => "Architect Adept",
			Rank::GrandArchitect => "Grand Architect",
			Rank::FreeMaster => "Free Master",
			Rank::MasterConstant => "Master Constant",
			Rank::GrandMaster => "Grand Master",
		}
	}

	fn possible_values() -> Vec<Self> {
		vec![
			Rank::Candidate,
			Rank::Humble,
			Rank::Proficient,
			Rank::Fellow,
			Rank::Architect,
			Rank::ArchitectAdept,
			Rank::GrandArchitect,
			Rank::FreeMaster,
			Rank::MasterConstant,
			Rank::GrandMaster,
		]
	}
}

#[test]
fn parses_example_file() {
	let file = std::fs::read_to_string("example/example.evidence").unwrap();
	let evidence: FellowshipReport = serde_yaml::from_str(&file).unwrap();

	assert_eq!(evidence.name, "Max Power");
}
