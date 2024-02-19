#[derive(sailfish::TemplateOnce)]
#[template(path = "evidence.stpl")]
pub struct EvidenceTemplate<Rank: crate::traits::Rank> {
	pub report: crate::evidence::EvidenceReport<Rank>,
}
