use serde::{Deserialize, Serialize};
use crate::collective::CollectiveId;
use crate::collective::Collective;

#[derive(Debug, Serialize, Deserialize)]
pub struct Member<C: Collective> {
	pub name: String,
	pub address: String,
	pub github: String,
	pub collective: CollectiveId,
	pub rank: C::Rank,
}

/// Con-empty collection of [Member]s.
#[derive(Debug, Serialize, Deserialize)]
pub struct Members<C: Collective> {
	pub members: Vec<Member<C>>,
	pub collective: CollectiveId,
}

impl<C: Collective> TryFrom<Vec<Member<C>>> for Members<C> {
	type Error = &'static str;

	fn try_from(members: Vec<Member<C>>) -> Result<Self, Self::Error> {
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
