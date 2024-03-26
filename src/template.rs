use crate::collective::Collective;

#[derive(sailfish::TemplateOnce)]
#[template(path = "evidence.html.stpl")]
pub struct EvidenceTemplate<C: Collective> {
	pub report: crate::evidence::EvidenceReport<C>,
}
#[derive(sailfish::TemplateOnce)]
#[template(path = "members.md.stpl")]
pub struct MembersTemplate<C: Collective> {
	pub members: crate::member::Members<C>,
}
