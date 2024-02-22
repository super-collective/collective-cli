use crate::{cmd::plural, fellowship::FellowshipReport};

use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, clap::Parser)]
pub struct IndexEvidenceCommand {
	/// The evidence folder.
	#[clap(long, default_value = "evidence")]
	evidence: PathBuf,

	/// By what to index.
	#[clap(long, default_value = "date,reporter", value_delimiter = ',')]
	by: Vec<IndexMetric>,

	/// Force re-index everything and delete all created in the index folders.
	#[clap(long)]
	reindex: bool,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum IndexMetric {
	/// By date.
	Date,
	/// By reporter.
	Reporter,
}

impl IndexEvidenceCommand {
	pub fn run(&self) -> Result<()> {
		self.check_root_folder()?;

		self.create_date_index()?;
		self.create_reporter_index()?;

		let evidences = self.parse_files()?;
		let (mut scanned, mut created) = (0, 0);

		for (path, evidence) in evidences.iter() {
			let file_name = path.file_name().ok_or("No file name")?;
			log::debug!("Processing '{}'", path.display());

			if self.by_reporter() {
				let sub = self.create_sub_folder(&["by_reporter", &evidence.canonical_name()])?;

				// yikes... this is a bit ugly
				let from = PathBuf::from("..").join("..").join(file_name);
				let to = sub.join(file_name);

				if std::fs::symlink_metadata(&to).is_ok() {
					// NOTE: we cannot easily check the content of the link here bc of
					// cross-platform.
					log::info!("Link '{}' already exists", to.display());
				} else {
					log::info!("Linking to '{}' as '{}'", to.display(), from.display());
					symlink::symlink_file(&from, &to)?;
					created += 1;
				}
				scanned += 1;
			}

			if self.by_date() {
				let splits = evidence.date.split('-').collect::<Vec<_>>();
				let sub =
					self.create_sub_folder(&["by_date", (splits[0]), (splits[1]), (splits[2])])?;

				// yikes... this is a bit ugly
				let from = PathBuf::from("..").join("..").join("..").join("..").join(file_name);
				let to = sub.join(file_name);

				if std::fs::symlink_metadata(&to).is_ok() {
					// NOTE: we cannot easily check the content of the link here bc of
					// cross-platform.
					log::info!("Link '{}' already exists", to.display());
				} else {
					log::info!("Linking to '{}' as '{}'", to.display(), from.display());
					symlink::symlink_file(&from, &to)?;
					created += 1;
				}
				scanned += 1;
			}
		}

		println!("Scanned {} and created {} new link{}.", scanned, created, plural(created));

		Ok(())
	}

	fn check_root_folder(&self) -> Result<()> {
		let root = PathBuf::from(&self.evidence);

		if !root.exists() {
			return Err(format!("Folder '{}' does not exist", root.display()).into());
		}
		if !root.is_dir() {
			return Err(format!("Folder '{}' is not a directory", root.display()).into());
		}

		Ok(())
	}

	fn parse_files(&self) -> Result<Vec<(PathBuf, FellowshipReport)>> {
		let mut reports = vec![];

		for entry in std::fs::read_dir(&self.evidence)? {
			let entry = entry?;
			let path = entry.path();

			if path.is_file() && path.extension() == Some("evidence".as_ref()) {
				let file = std::fs::File::open(&path)?;
				let report: FellowshipReport = serde_yaml::from_reader(file)?;

				log::debug!("Parsed evidence report from '{}'", path.display());
				reports.push((path, report));
			}
		}

		Ok(reports)
	}

	fn create_date_index(&self) -> Result<()> {
		if !self.by_date() {
			return Ok(());
		}
		if self.reindex {
			self.delete_sub_folder(&["by_date"])?;
		}
		let _folder = self.create_sub_folder(&["by_date"])?;

		Ok(())
	}

	fn create_reporter_index(&self) -> Result<()> {
		if !self.by_reporter() {
			return Ok(());
		}
		if self.reindex {
			self.delete_sub_folder(&["by_reporter"])?;
		}
		let _folder = self.create_sub_folder(&["by_reporter"])?;

		Ok(())
	}

	fn delete_sub_folder(&self, subs: &[&str]) -> Result<()> {
		let folder = self.create_sub_folder(subs)?;

		log::info!("Deleting index folder at '{}'", folder.display());
		std::fs::remove_dir_all(&folder)?;

		Ok(())
	}

	fn create_sub_folder(&self, subs: &[&str]) -> Result<PathBuf> {
		let mut folder = self.evidence.clone();
		for sub in subs {
			folder = folder.join(sub);
		}

		if folder.exists() {
			log::debug!("Index folder '{}' already exists", folder.display());

			if !folder.is_dir() {
				return Err(format!("Path {} is not a directory", folder.display()).into());
			}
		} else {
			log::info!("Creating index folder at '{}'", folder.display());
			std::fs::create_dir_all(&folder)?;
		}

		Ok(folder)
	}

	fn by_date(&self) -> bool {
		self.by.contains(&IndexMetric::Date)
	}

	fn by_reporter(&self) -> bool {
		self.by.contains(&IndexMetric::Reporter)
	}
}
