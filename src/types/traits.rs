// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use anyhow::Result;
use inquire::{Confirm, Select};
use std::borrow::Cow;

pub trait Named {
	fn name(&self) -> Cow<'static, str>;
}

pub trait Numbered {
	fn number(&self) -> u8;
}

impl<T: EnumLike + core::cmp::PartialEq> Numbered for T {
	fn number(&self) -> u8 {
		T::variants().iter().position(|r| r == self).unwrap() as u8
	}
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

impl<T: ?Sized + MultiTierNamed> Named for T {
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
			self.strip_prefix("https://github.com/polkadot-fellows/runtimes/pull/").or(self.strip_prefix("https://github.com/polkadot-fellows/runtimes/issues/"))
		{
			let link = link.replace("/pull/", "#").replace("/issues/", "#");
			format!("Runtime {}", link)
		} else if self.contains("/pulls?q=") {
			"PR range".to_string()
		} else {
			let s = self.trim_start_matches("https://");
			// We need to use `chars()` to not accidentally intersect a multi-byte UTF8 sequence.
			s.chars().take(25).collect()
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

pub trait Schema {
	fn schema() -> String;
}

impl<T: schemars::JsonSchema> Schema for T {
	fn schema() -> String {
		let schema = schemars::schema_for!(T);
		serde_json::to_string_pretty(&schema).unwrap()
	}
}

pub trait IntoRomanNumeral {
	fn into_roman_numeral(self) -> String;
}

impl<T: Into<u32>> IntoRomanNumeral for T {
	fn into_roman_numeral(self) -> String {
		let n: u32 = self.into();
		if n > 9 {
			return n.to_string();
		}

		[
			"0", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X",
		][n as usize].to_string()
	}
}
