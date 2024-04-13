use crate::{
	collective::{fellowship::FellowshipCollective, potoc::PotocCollective},
	types::prelude::*,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectiveId {
	#[serde(alias = "fellowship")]
	Fellowship,
	#[serde(alias = "potoc")]
	Potoc,
}

impl Named for CollectiveId {
	fn name(&self) -> Cow<'static, str> {
		match self {
			Self::Fellowship => FellowshipCollective::NAME.into(),
			Self::Potoc => PotocCollective::NAME.into(),
		}
	}
}
