use super::Result;

use std::{collections::BTreeMap, path::PathBuf};

pub struct Cache {
	path: PathBuf,
	values: BTreeMap<String, String>,
}

impl Cache {
	pub fn load() -> Result<Self> {
		let path = Self::config_path()?;

		match Self::from_path(path.clone()) {
			Ok(c) => Ok(c),
			Err(_) => Ok(Self { path, values: Default::default() }),
		}
	}

	pub fn flush(&self) -> Result<()> {
		std::fs::create_dir_all(self.path.parent().unwrap())?;
		let file = std::fs::File::create(&self.path)?;
		serde_yaml::to_writer(file, &self.values)?;
		log::info!("Persisting changes {}", self.path.display());

		Ok(())
	}

	pub fn from_path(path: PathBuf) -> Result<Self> {
		let file = std::fs::File::open(&path)?;
		let values = serde_yaml::from_reader(file)?;

		log::info!("Restored config from {:?}", path.display());

		Ok(Self { path, values })
	}

	fn config_path() -> Result<PathBuf> {
		let Some(root) = dirs::config_dir() else {
			return Err("Could not find the config directory".into());
		};

		Ok(root.join("collective-cli").join("config.toml"))
	}

	pub fn insert(&mut self, k: &str, value: String) {
		self.values.insert(k.into(), value);
	}

	pub fn try_get(&self, k: &str) -> Result<Option<String>> {
		let v = match self.values.get(k) {
			Some(v) => v,
			None => return Ok(None),
		};

		let ret = serde_yaml::from_str(v)?;
		Ok(Some(ret))
	}
}
