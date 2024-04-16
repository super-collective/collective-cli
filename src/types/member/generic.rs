use crate::{prompt::Prompt, types::prelude::*};
use core::fmt::Debug;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// TODO split into member and ranked-member
#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug(bound = "C::Rank: Debug"))]
#[derivative(Clone(bound = "C::Rank: Clone"))]
pub struct GenericMember<C: CollectiveTrait> {
	pub name: String,
	pub address: String,
	pub github: String,
	pub rank: C::Rank,
}

impl<C: CollectiveTrait> MemberTrait for GenericMember<C> {
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

impl<C: CollectiveTrait> Encode for GenericMember<C> {
	fn to_yaml(&self) -> serde_yaml::Value {
		serde_yaml::to_value(self).unwrap()
	}
}

impl<C: CollectiveTrait> Decode for GenericMember<C> {
	fn from_yaml(value: serde_yaml::Value) -> anyhow::Result<Self> {
		serde_yaml::from_value(value).map_err(Into::into)
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

		let rank_title =
			if let Some(title) = title { format!("Rank to {title}") } else { "Rank".into() };
		let rank = C::Rank::query(Some(&rank_title), None, prompt)?;

		Ok(Self { name, address, github, rank })
	}
}
