// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{
	collective::{fellowship::FellowshipCollective, potoc::PotocCollective},
	types::prelude::*,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(
	Debug,
	Serialize,
	Deserialize,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	clap::ValueEnum,
	strum::EnumIter,
)]
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
