use crate::types::{
	prelude::Tasks,
	traits::{EnumLike, MultiTierNamed, Query},
};
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

pub trait EvidenceTrait {
	fn title(&self) -> &str;
	fn tasks(&self) -> &Vec<Tasks>;
	fn category(&self) -> &dyn EvidenceCategoriesBaseTrait;
}

// Object safe version of EvidenceCategories.
pub trait EvidenceCategoriesBaseTrait: MultiTierNamed + EnumLike + Debug {}

// Not object safe version of EvidenceCategories.
pub trait EvidenceCategories:
	EvidenceCategoriesBaseTrait + Copy + Query + Serialize + DeserializeOwned + Ord
{
}

impl<T: EvidenceCategoriesBaseTrait + Copy + Query + Serialize + DeserializeOwned + Ord>
	EvidenceCategories for T
{
}
