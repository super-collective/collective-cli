#[derive(sailfish::TemplateOnce)]
#[template(path = "evidence.html.stpl")]
pub struct EvidenceTemplate<Rank: crate::traits::Rank> {
	pub report: crate::evidence::EvidenceReport<Rank>,
}
#[derive(sailfish::TemplateOnce)]
#[template(path = "members.md.stpl")]
pub struct MembersTemplate<Rank: crate::traits::Rank> {
	pub members: crate::member::Members<Rank>,
}
