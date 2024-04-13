use crate::types::prelude::*;
use inquire::Select;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "intent", content = "rank", rename_all = "lowercase")]
pub enum Wish<R> {
	Retain(R),
	Promote(R),
}

impl<R: Rank> Query for Wish<R> {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		p: &mut crate::prompt::Prompt,
	) -> anyhow::Result<Self> {
		let options = vec!["retain", "promote"];
		let wish = Select::new("Wish", options.clone()).prompt()?;
		let rank_title = format!("Rank to {}", wish);
		let rank = R::query(Some(&rank_title), None, p)?;

		Ok(match wish {
			"retain" => Self::Retain(rank),
			"promote" => Self::Promote(rank),
			_ => unreachable!(),
		})
	}
}

impl<Rank> Wish<Rank> {
	pub fn title(&self) -> &'static str {
		match self {
			Wish::Retain(_) => "Retain",
			Wish::Promote(_) => "Promote",
		}
	}
}
