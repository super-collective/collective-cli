use crate::types::traits::{Named, RankBaseTrait};

pub trait MemberTrait: Named {
	fn github(&self) -> &str;
	fn address(&self) -> &str;
	fn rank(&self) -> &dyn RankBaseTrait;
}
