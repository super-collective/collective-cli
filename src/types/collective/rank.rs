// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::*;
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

/// Object safe version of a Rank.
pub trait RankBaseTrait: Named + Numbered + EnumLike {}

/// Not object safe version of a Rank.
pub trait Rank:
	RankBaseTrait + Copy + Debug + Into<u32> + Serialize + DeserializeOwned + schemars::JsonSchema
{
}

impl<
		T: RankBaseTrait
			+ Copy
			+ Debug
			+ Into<u32>
			+ Serialize
			+ DeserializeOwned
			+ schemars::JsonSchema,
	> Rank for T
{
}
