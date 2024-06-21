// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{types::prelude::*, using_collective};
use core::fmt::Debug;

use super::MemberTrait;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Clone, schemars::JsonSchema)]
#[serde(tag = "collective")]
pub enum Member {
	Fellowship(<crate::collective::fellowship::FellowshipCollective as CollectiveTrait>::Member),
	Potoc(<crate::collective::potoc::PotocCollective as CollectiveTrait>::Member),
}

impl Member {
	pub fn collective(&self) -> CollectiveId {
		match self {
			Self::Fellowship(_) => CollectiveId::Fellowship,
			Self::Potoc(_) => CollectiveId::Potoc,
		}
	}
}

impl Named for Member {
	fn name(&self) -> Cow<'static, str> {
		match self {
			Self::Fellowship(member) => member.name(),
			Self::Potoc(member) => member.name(),
		}
	}
}

impl MemberTrait for Member {
	fn matrix(&self) -> &str {
		using_collective!(self, member, { member.matrix() })
	}
	
	fn github(&self) -> &str {
		using_collective!(self, member, { member.github() })
	}

	fn address(&self) -> &str {
		using_collective!(self, member, { member.address() })
	}

	fn rank(&self) -> &dyn RankBaseTrait {
		using_collective!(self, member, { member.rank() })
	}
}
