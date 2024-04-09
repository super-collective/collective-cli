pub mod fellowship;
pub mod potoc;

use derivative::Derivative;
use crate::{
	evidence::Evidence,
	traits::{MultiTierNamed, Query, Rank},
};
use crate::traits::vector_prompt;
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::borrow::Cow;
use crate::traits::EnumLike;

pub trait Collective {
	const ID: CollectiveId;

	type Rank: Rank;
	type EvidenceCategories: EvidenceCategories;
	type Member: Debug + Serialize + DeserializeOwned + Clone + Query;
	const NAME: &'static str;
	const NICKNAME: &'static str;
}

// Object safe version of EvidenceCategories.
pub trait EvidenceCategoriesBaseTrait : MultiTierNamed + EnumLike + Debug
{ }

// Not object safe version of EvidenceCategories.
pub trait EvidenceCategories:
	EvidenceCategoriesBaseTrait + Copy + Query + Serialize + DeserializeOwned + Ord
{ }

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

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug(bound = "C::Member: Debug, C::EvidenceCategories: Debug"))]
#[derivative(Clone(bound = "C::Member: Clone, C::EvidenceCategories: Debug"))]
pub struct GenericJoinRequest<C: Collective> {
	pub member: C::Member,
	pub date: String,
	pub evidence: Vec<Evidence<C::EvidenceCategories>>,
}

impl<C: Collective> Query for GenericJoinRequest<C> {
	fn query(_title: Option<&str>, _key: Option<&str>, p: &mut crate::prompt::Prompt) -> anyhow::Result<Self> {
		let member = C::Member::query(Some("join"), None, p)?;
		let date = p.query_date("Date of this Request")?;
		println!("\nYou will now be queried to provide evidence for your request.\nThis should show concrete examples how you contributed to the mission of the collective.\nEvery piece of evidence has a title, category and a set of tasks.\nPlease fill them in either in this CLI or in the resulting yaml file that will be created at the end of this prompt.\n");
		let evidence = vector_prompt("evidence", || Evidence::<C::EvidenceCategories>::query(Some("are a good addition to the fellowship"), None, p))?;

		Ok(Self {
			member,
			date: date.to_string(),
			evidence,
		})
	}
}
