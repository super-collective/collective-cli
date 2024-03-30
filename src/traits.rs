use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use inquire::Select;

#[macro_export]
macro_rules! using_collective {
	( $c:expr, $inner:ident, $e:expr ) => {
		match $c {
			Self::Fellowship($inner) => $e,
			Self::Potoc($inner) => $e,
		}
	};
}

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
	fn variants() -> Vec<Self> where Self: Sized;
}

impl<T: EnumLike + Named + Clone> Query for T {
	fn query(title: Option<&str>, _key: Option<&str>, _p: &mut crate::prompt::Prompt) -> anyhow::Result<Self> {
		let title = title.expect("Blanket implemented query for named types always need a title");
		assert!(_key.is_none(), "Blanket implemented query for named types does not support keys");

		let variants = T::variants();
		let options = variants
			.iter()
			.enumerate()
			.map(|(i, r)| r.name())
			.collect::<Vec<_>>();
		let rank = Select::new(title, options.clone()).prompt()?;
		let index = options.iter().position(|r| r == &rank).unwrap();

		Ok(variants[index].clone())
	}
}

pub trait Rank: Named + EnumLike + Into<u32> + Copy + Debug + Serialize + for<'a> Deserialize<'a> {}

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
		} else if let Some(link) = self.strip_prefix("https://github.com/polkadot-fellows/") {
			let link = link.replace("/pull/", "#").replace("/issues/", "#");
			format!("Fellowship {}", link)
		} else {
			self.clone()
		}
	}
}

pub trait Query {
	fn query(_title: Option<&str>, _key: Option<&str>, p: &mut crate::prompt::Prompt) -> anyhow::Result<Self>
	where
		Self: Sized;
	
	fn query_bare(p: &mut crate::prompt::Prompt) -> anyhow::Result<Self>
	where
		Self: Sized
	{
		Self::query(None, None, p)
	}
}
