// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{prompt::Prompt, types::prelude::*};
use core::fmt::Debug;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Info about an existing or aspiring member of a collective.
// TODO split into member and ranked-member
#[derive(Serialize, Deserialize, Derivative, schemars::JsonSchema)]
#[derivative(Debug(bound = "C::Rank: Debug"))]
#[derivative(Clone(bound = "C::Rank: Clone"))]
#[schemars(bound = "", rename = "Member")]
pub struct GenericMember<C: CollectiveTrait> {
	/// Legal name or pseudonym of the member.
	pub name: String,
	/// On-chain address of the member.
	///
	/// Can be a Substrate, Polkadot or Collectives Address.
	pub address: String,
	/// GitHub handle of the member.
	pub github: String,
	/// Matrix chat handle.
	pub matrix: String,
	/// Current rank of the member.
	pub rank: C::Rank,
}

impl<C: CollectiveTrait> MemberTrait for GenericMember<C> {
	fn matrix(&self) -> &str {
		&self.matrix
	}
	
	fn github(&self) -> &str {
		&self.github
	}

	fn address(&self) -> &str {
		&self.address
	}

	fn rank(&self) -> &dyn RankBaseTrait {
		&self.rank
	}
}

impl<C: CollectiveTrait> Named for GenericMember<C> {
	fn name(&self) -> Cow<'static, str> {
		self.name.clone().into()
	}
}

impl<C: CollectiveTrait> Query for GenericMember<C> {
	fn query(title: Option<&str>, _key: Option<&str>, prompt: &mut Prompt) -> anyhow::Result<Self> {
		let name = prompt.query_cached_text::<String>(
			"reporter_legal_name",
			"legal name or pseudonym",
			None,
		)?;

		let address = prompt.query_cached_text::<String>(
			"reporter_address",
			"your Polkadot address",
			None,
		)?;

		let github = prompt
			.query_cached_text::<String>("reporter_github", "your GitHub handle", None)?
			.replace('@', " ");

		let matrix = prompt.query_cached_text::<String>(
			"reporter_matrix",
			"your Matrix chat handle",
			None,
		)?;

		let rank_title =
			if let Some(title) = title { format!("Rank {title}") } else { "Rank".into() };
		let rank = C::Rank::query(Some(&rank_title), None, prompt)?;

		Ok(Self { name, address, github, matrix, rank })
	}
}
