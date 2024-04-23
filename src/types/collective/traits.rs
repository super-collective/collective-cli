// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::*;
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

pub trait CollectiveTrait {
	const ID: CollectiveId;
	const MANIFESTO: &'static str;

	type Rank: Rank;
	type EvidenceCategories: EvidenceCategories;
	type Member: Debug + Serialize + DeserializeOwned + Clone + Query + schemars::JsonSchema;
	const NAME: &'static str;
	const NICKNAME: &'static str;
}
