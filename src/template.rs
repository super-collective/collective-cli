// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::*;

#[derive(sailfish::TemplateOnce)]
#[template(path = "evidence.html.stpl")]
pub struct EvidenceTemplate {
	pub report: EvidenceReport,
}

#[derive(sailfish::TemplateOnce)]
#[template(path = "evidence.md.stpl")]
pub struct EvidenceMdTemplate {
	pub report: EvidenceReport,
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
