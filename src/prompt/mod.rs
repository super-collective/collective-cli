mod cache;

use cache::Cache;
use inquire::Text;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Prompt {
	cache: Cache,
}

impl Prompt {
	pub fn new() -> Result<Self> {
		let cache = Cache::load()?;
		Ok(Self { cache })
	}

	pub fn query_cached_text<T: TryFrom<String>>(
		&mut self,
		key: &str,
		desc: &str,
		help: Option<&str>,
	) -> Result<T>
	where
		<T as TryFrom<String>>::Error: std::error::Error + 'static,
	{
		let mut default = String::new();
		if let Ok(Some(found)) = self.cache.try_get(key) {
			if T::try_from(found.clone()).is_ok() {
				default = found;
			}
		}

		let query = format!("Please enter {desc}:");
		let value = loop {
			let prompt = Text::new(&query).with_initial_value(&default);

			let value = if let Some(help) = help { prompt.with_help_message(help) } else { prompt }
				.prompt()?;

			if !value.is_empty() {
				break value;
			}
			println!("Value cannot be empty");
		};
		let decoded = T::try_from(value.clone())?;

		self.cache.insert(key, value);
		self.cache.flush()?;

		Ok(decoded)
	}
}
