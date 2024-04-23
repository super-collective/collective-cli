use crate::types::prelude::*;
use core::fmt::Debug;
use derivative::Derivative;
use inquire::Text;
use serde::{Deserialize, Serialize};

/// A request to join a collective.
#[derive(Derivative, Serialize, Deserialize, schemars::JsonSchema)]
#[derivative(Debug(bound = "C::Member: Debug, C::EvidenceCategories: Debug"))]
#[derivative(Clone(bound = "C::Member: Clone, C::EvidenceCategories: Debug"))]
#[schemars(bound = "", rename = "JoinRequest")]
pub struct GenericJoinRequest<C: CollectiveTrait> {
	/// Info about the requester that wants to join.
	pub member: C::Member,
	/// Motivation of the requester for joining the collective.
	pub motivation: String,
	/// Bio/about section of the requester.
	pub about: String,
	/// Date of the request.
	pub date: String,
	/// Evidence to underpin the request.
	pub evidence: Vec<GenericEvidence<C::EvidenceCategories>>,
}

impl<C: CollectiveTrait> Query for GenericJoinRequest<C> {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		p: &mut crate::prompt::Prompt,
	) -> anyhow::Result<Self> {
		let member = C::Member::query(Some("to join"), None, p)?;
		let motivation = Text::new("What motivated you to apply?").prompt()?;
		let about = Text::new("Tell us a bit about yourself.").prompt()?;
		let date = p.query_date("Date of this Request")?;
		println!("\nYou will now be queried to provide evidence for your request.\nThis should show concrete examples how you contributed to the mission of the collective.\nEvery piece of evidence has a title, category and a set of tasks.\nPlease fill them in either in this CLI or in the resulting yaml file that will be created at the end of this prompt.\n");
		let evidence = vector_prompt("evidence", || {
			GenericEvidence::<C::EvidenceCategories>::query(
				Some("are a good addition to the fellowship"),
				None,
				p,
			)
		})?;

		Ok(Self { member, motivation, about, date: date.to_string(), evidence })
	}
}
