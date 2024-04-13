mod cache;

use cache::Cache;
use chrono::{NaiveDate, Weekday};
use inquire::{DateSelect, Text};

pub type Result<T> = anyhow::Result<T>;

pub struct Prompt {
	cache: Option<Cache>,
}

impl Prompt {
	pub fn new(cache: bool) -> Result<Self> {
		let cache = if cache { Some(Cache::load()?) } else { None };
		Ok(Self { cache })
	}

	pub fn query_cached_text<T: TryFrom<String>>(
		&mut self,
		key: &str,
		desc: &str,
		help: Option<&str>,
	) -> Result<T>
	where
		<T as TryFrom<String>>::Error: std::error::Error + Send + Sync + 'static,
	{
		let mut default = String::new();
		if let Some(ref cache) = self.cache {
			if let Ok(Some(found)) = cache.try_get(key) {
				if T::try_from(found.clone()).is_ok() {
					default = found;
				}
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

		if let Some(ref mut cache) = self.cache {
			cache.insert(key, value);
			cache.flush()?;
		}

		Ok(decoded)
	}

	pub fn query_date(&self, title: &str) -> Result<NaiveDate> {
		DateSelect::new(title)
			.with_starting_date(chrono::Utc::now().date_naive())
			.with_week_start(Weekday::Mon)
			.prompt()
			.map_err(Into::into)
	}
}
