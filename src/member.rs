use serde::{Deserialize, Serialize};
use crate::evidence::Collective; // TODO refactor

#[derive(Debug, Serialize, Deserialize)]
pub struct Member<Rank> {
	pub name: String,
	pub address: String,
	pub github: String,
	pub collective: Collective,
	pub rank: Rank,
}

/// Con-empty collection of [Member]s.
#[derive(Debug, Serialize, Deserialize)]
pub struct Members<Rank> {
	pub members: Vec<Member<Rank>>,
	pub collective: Collective,
}

impl<Rank> TryFrom<Vec<Member<Rank>>> for Members<Rank> {
	type Error = &'static str;

	fn try_from(members: Vec<Member<Rank>>) -> Result<Self, Self::Error> {
		if members.is_empty() {
			Err("Members collection cannot be empty")
		} else {
			Ok(Self {
				collective: members[0].collective.clone(),
				members,
			})
		}
	}
}
