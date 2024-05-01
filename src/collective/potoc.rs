// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::borrow::Cow;

/// The Polkadot Tooling Collective (PoToC).
pub struct PotocCollective;

impl CollectiveTrait for PotocCollective {
	const NAME: &'static str = "Tooling Collective";
	const NICKNAME: &'static str = "PoToC";
	const ID: CollectiveId = CollectiveId::Fellowship;
	const MANIFESTO: &'static str = "https://github.com/polkadot-tooling-collective/constitution";

	type Rank = PotocRank;
	type EvidenceCategories = PotocEvidenceCategory;
	type Member = PotocMember;
}

pub type PotocMember = GenericMember<PotocCollective>;
pub type PotocEvidenceReport = GenericEvidenceReport<PotocCollective>;
pub type PotocJoinRequest = GenericJoinRequest<PotocCollective>;

#[repr(u8)]
#[derive(
	Debug, Serialize_repr, Deserialize_repr, Copy, Clone, strum::EnumIter, schemars::JsonSchema,
)]
pub enum PotocRank {
	Candidate = 0,
	Member = 1,
}

impl Named for PotocRank {
	fn name(&self) -> Cow<'static, str> {
		match self {
			Self::Candidate => "Candidate",
			Self::Member => "Member",
		}
		.into()
	}
}

impl RankBaseTrait for PotocRank {}

impl From<PotocRank> for u32 {
	fn from(rank: PotocRank) -> u32 {
		rank as u32
	}
}

#[derive(
	Debug, Serialize, Deserialize, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum PotocEvidenceCategory {
	CoreTooling,
	DAppTooling,
}

impl EvidenceCategoriesBaseTrait for PotocEvidenceCategory {}

impl MultiTierNamed for PotocEvidenceCategory {
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
