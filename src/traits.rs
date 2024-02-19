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

pub trait Rank: Copy {
	fn name(&self) -> &'static str;
	fn possible_values() -> Vec<Self>;
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
