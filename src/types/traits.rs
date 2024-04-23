// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use anyhow::Result;
use inquire::{Confirm, Select};
use std::borrow::Cow;

/// Encode an item.
///
/// Only needed since the serde traits are not object safe.
pub trait Encode {
	fn to_yaml(&self) -> serde_yaml::Value;
}

/// Decode an item.
///
/// Only needed since the serde traits are not object safe.
pub trait Decode {
	fn from_yaml(value: serde_yaml::Value) -> anyhow::Result<Self>
	where
		Self: Sized;
}

pub trait Named {
	fn name(&self) -> Cow<'static, str>;
}

/// Something with a fixed set of variants that can be listed at runtime.
pub trait EnumLike {
	/// All possible variants.
	fn variants() -> Vec<Self>
	where
		Self: Sized;
}

impl<T: EnumLike + Named + Clone> Query for T {
	fn query(
		title: Option<&str>,
		_key: Option<&str>,
		_p: &mut crate::prompt::Prompt,
	) -> Result<Self> {
		let title = title.expect("Blanket implemented query for named types always need a title");
		assert!(_key.is_none(), "Blanket implemented query for named types does not support keys");

		let variants = T::variants();
		let options = variants.iter().map(Named::name).collect::<Vec<_>>();
		let rank = Select::new(title, options.clone()).prompt()?;
		let index = options.iter().position(|r| r == &rank).unwrap();

		Ok(variants[index].clone())
	}
}

impl<T: strum::IntoEnumIterator> EnumLike for T {
	fn variants() -> Vec<Self> {
		T::iter().collect()
	}
}

pub trait MultiTierNamed {
	fn multi_tier_name(&self) -> Vec<Cow<'static, str>>;
}

impl<T: MultiTierNamed> Named for T {
	fn name(&self) -> Cow<'static, str> {
		self.multi_tier_name().join("").into()
	}
}

// HTML stuff:

pub trait HumanDisplay {
	fn human(&self) -> String;
}

impl<T: HumanDisplay> HumanDisplay for Option<T> {
	fn human(&self) -> String {
		match self {
			Some(t) => t.human(),
			None => String::from("?"),
		}
	}
}

pub trait FormatLink {
	fn link_name(&self) -> String;

	fn format_link(&self) -> String
	where
		Self: core::fmt::Display,
	{
		format!("<a target=\"_blank\" href=\"{}\">{}</a>", self, self.link_name())
	}
}

impl FormatLink for String {
	fn link_name(&self) -> String {
		if let Some(sdk) = self.strip_prefix("https://github.com/paritytech/polkadot-sdk/pull/") {
			format!("SDK {}", sdk)
		} else if let Some(rfc) =
			self.strip_prefix("https://github.com/polkadot-fellows/RFCs/pull/")
		{
			format!("RFC {}", rfc)
		} else if let Some(link) =
			self.strip_prefix("https://github.com/polkadot-fellows/runtimes/pull/")
		{
			let link = link.replace("/pull/", "#").replace("/issues/", "#");
			format!("Runtimes {}", link)
		} else if self.contains("/pulls?q=") {
			"PR range".to_string()
		} else {
			self.clone().trim_start_matches("https://")[..25].into()
		}
	}
}

pub trait Query {
	fn query(
		_title: Option<&str>,
		_key: Option<&str>,
		p: &mut crate::prompt::Prompt,
	) -> anyhow::Result<Self>
	where
		Self: Sized;

	fn query_bare(p: &mut crate::prompt::Prompt) -> anyhow::Result<Self>
	where
		Self: Sized,
	{
		Self::query(None, None, p)
	}
}

pub fn vector_prompt<F: FnMut() -> std::result::Result<R, E>, R, E: Into<anyhow::Error>>(
	title: &str,
	mut f: F,
) -> Result<Vec<R>> {
	let mut ret = Vec::new();
	loop {
		if let Ok(value) = f() {
			ret.push(value);
		} else {
			println!("Invalid input, please try again.");
			continue;
		}

		if !Confirm::new(&format!("Add more {title}? (y/n)")).prompt()? {
			return Ok(ret);
		}
	}
}
