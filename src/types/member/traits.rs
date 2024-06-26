// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::*;

pub trait MemberTrait: Named {
	fn canonical_name(&self) -> String {
		self.name().to_string().replace(' ', "-").to_lowercase()
	}
	fn matrix(&self) -> &str;
	fn github(&self) -> &str;
	fn github_link(&self) -> String {
		format!(
			"<a target='_blank' href='https://github.com/{}'>{}</a>",
			self.github(),
			self.github()
		)
	}
	fn address(&self) -> &str;
	fn address_link(&self, shorten: bool) -> String {
		let address = self.address();
		let shortened =
			if shorten && address.len() > 8 { format!("{}..", &address[..8]) } else { address.to_string() };
		format!(
			"<a target='_blank' href='https://collectives.statescan.io/#/accounts/{}'>{}</a>",
			address, shortened
		)
	}
	fn rank(&self) -> &dyn RankBaseTrait;
}
