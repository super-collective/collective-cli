pub mod fellowship;
pub mod potoc;

use serde::{Deserialize, Serialize};
use crate::traits::Rank;

pub trait Collective {
	type Rank: Rank;
	const NAME: &'static str;
}

#[derive(Debug, Serialize, Deserialize, Clone, clap::ValueEnum)]
pub enum CollectiveId {
	#[serde(alias = "fellowship")]
	Fellowship,
	#[serde(alias = "potoc")]
	PoToC,
}

impl super::traits::Named for CollectiveId {
	fn name(&self) -> &'static str {
		match self {
			Self::Fellowship => fellowship::FellowshipCollective::NAME.into(),
			Self::PoToC => potoc::PotocCollective::NAME.into(),
		}
	}
}
