// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::*;
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub trait WishTrait: Named {
	fn rank(&self) -> &dyn RankBaseTrait;
	fn verb(&self) -> Cow<'static, str>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "intent", content = "rank", rename_all = "lowercase")]
pub enum Wish<R> {
	Retain(R),
	Promote(R),
}

impl<R: Rank> WishTrait for Wish<R> {
	fn rank(&self) -> &dyn RankBaseTrait {
		match self {
			Self::Retain(rank) => rank,
			Self::Promote(rank) => rank,
		}
	}

	fn verb(&self) -> Cow<'static, str> {
		Cow::Borrowed(match self {
			Self::Retain(_) => "the rank of",
			Self::Promote(_) => "to rank",
		})
	}
}

impl<R> Named for Wish<R> {
	fn name(&self) -> Cow<'static, str> {
		Cow::Borrowed(match self {
			Self::Retain(_) => "Retain",
			Self::Promote(_) => "Promote",
		})
	}
}

impl<R: Rank> Query for Wish<R> {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		p: &mut crate::prompt::Prompt,
	) -> anyhow::Result<Self> {
		let options = vec!["retain", "promote"];
		let wish = Select::new("Wish", options.clone()).prompt()?;
		let rank_title = format!("Rank {}", wish);
		let rank = R::query(Some(&rank_title), None, p)?;

		Ok(match wish {
			"retain" => Self::Retain(rank),
			"promote" => Self::Promote(rank),
			_ => unreachable!(),
		})
	}
}
