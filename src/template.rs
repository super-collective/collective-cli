use crate::types::prelude::*;

#[derive(sailfish::TemplateOnce)]
#[template(path = "evidence.html.stpl")]
pub struct EvidenceTemplate<C: Collective> {
	pub report: EvidenceReport<C>,
}

#[derive(sailfish::TemplateOnce)]
#[template(path = "members.md.stpl")]
pub struct MembersTemplate {
	pub members: Members,
}

#[derive(sailfish::TemplateOnce)]
#[template(path = "join_request.md.stpl")]
pub struct JoinRequestTemplate {
	pub request: JoinRequest,
}

#[derive(sailfish::TemplateOnce)]
#[template(path = "join_request.html.stpl")]
pub struct JoinRequestTemplateHtml {
	pub request: JoinRequest,
}
