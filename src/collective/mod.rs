pub mod fellowship;
pub mod potoc;

use crate::{
	traits::{Query, Rank},
	types::evidence::EvidenceCategories,
};

use crate::traits::EnumLike;
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::borrow::Cow;

pub trait Collective {
	const ID: CollectiveId;

	type Rank: Rank;
	type EvidenceCategories: EvidenceCategories;
	type Member: Debug + Serialize + DeserializeOwned + Clone + Query;
	const NAME: &'static str;
	const NICKNAME: &'static str;
}

#[derive(Debug, Serialize, Deserialize, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectiveId {
	#[serde(alias = "fellowship")]
	Fellowship,
	#[serde(alias = "potoc")]
	Potoc,
}

impl EnumLike for CollectiveId {
	fn variants() -> Vec<Self> {
		vec![Self::Fellowship, Self::Potoc]
	}
}

impl super::traits::Named for CollectiveId {
	fn name(&self) -> Cow<'static, str> {
		match self {
			Self::Fellowship => fellowship::FellowshipCollective::NAME.into(),
			Self::Potoc => potoc::PotocCollective::NAME.into(),
		}
	}
}
