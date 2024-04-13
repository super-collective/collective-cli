use crate::types::prelude::*;
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

pub trait Collective {
	const ID: CollectiveId;

	type Rank: Rank;
	type EvidenceCategories: EvidenceCategories;
	type Member: Debug + Serialize + DeserializeOwned + Clone + Query;
	const NAME: &'static str;
	const NICKNAME: &'static str;
}
