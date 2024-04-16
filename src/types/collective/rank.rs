use crate::types::prelude::*;
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

/// Object safe version of a Rank.
pub trait RankBaseTrait: Named + EnumLike {}

/// Not object safe version of a Rank.
pub trait Rank: RankBaseTrait + Copy + Debug + Into<u32> + Serialize + DeserializeOwned {}

impl<T: RankBaseTrait + Copy + Debug + Into<u32> + Serialize + DeserializeOwned> Rank for T {}
