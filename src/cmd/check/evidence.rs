use crate::fellowship::FellowshipReport;
use std::path::PathBuf;

use valico::{json_schema, json_schema::schema::ScopedSchema};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, clap::Parser)]
pub struct CheckEvidenceCommand {
	/// List of paths to evidence reports.
	#[clap(index = 1)]
	path: Vec<PathBuf>,
}

impl CheckEvidenceCommand {
	pub fn run(&self) -> Result<()> {
		let schema_str = FellowshipReport::schema();
		let schema: serde_json::Value = serde_json::from_str(&schema_str)?;
		let mut scope = json_schema::Scope::new();
		let schema = scope.compile_and_return(schema, false).unwrap();

		for path in &self.path {
			let data = std::fs::read_to_string(path.as_path())?;

			// Check that we can decode it.
			let _: FellowshipReport = serde_yaml::from_str(&data)?;

			// Check that it validates against the schema.
			Self::validate_schema(&schema, &data)?;
		}

		println!(
			"Validated {} evidence report{}.",
			self.path.len(),
			crate::cmd::plural(self.path.len())
		);

		Ok(())
	}

	fn validate_schema<'a>(schema: &'a ScopedSchema<'a>, data: &str) -> Result<()> {
		let mut doc_as_yaml: serde_yaml::Value = serde_yaml::from_str(data)?;
		doc_as_yaml.apply_merge()?;

		let doc_as_json: serde_json::Value =
			serde_yaml::from_value(serde_yaml::to_value(&doc_as_yaml)?)?;

		let res = schema.validate(&doc_as_json);

		if !res.is_valid() || !res.is_strictly_valid() {
			eprintln!("Validation failed for {:#?}", res);
		}

		Ok(())
	}
}
