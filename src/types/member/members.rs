use crate::{collective::CollectiveId, types::prelude::Member};

// Non-empty collection of [Member]s.
#[derive(Debug)]
pub struct Members {
	pub members: Vec<Member>,
	pub collective: CollectiveId,
}

impl TryFrom<Vec<Member>> for Members {
	type Error = &'static str;

	fn try_from(members: Vec<Member>) -> Result<Self, Self::Error> {
		if members.is_empty() {
			Err("Members collection cannot be empty")
		} else {
			Ok(Self { collective: members[0].collective(), members })
		}
	}
}
