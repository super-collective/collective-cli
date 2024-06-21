// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::{
	config::GlobalConfig,
	types::prelude::*,
};
use anyhow::Context;
use glob::glob;
use std::path::PathBuf;
use valico::{json_schema, json_schema::schema::ScopedSchema};

type Result<T> = anyhow::Result<T>;

#[derive(Debug, clap::Parser)]
pub struct CheckMemberCommand {
	/// Explicit list of paths to evidence reports.
	#[clap(long)]
	files: Option<Vec<PathBuf>>,
}

impl CheckMemberCommand {
	pub fn run(&self, g: &GlobalConfig) -> Result<()> {
		let schema_str = Member::schema();
		let schema: serde_json::Value = serde_json::from_str(&schema_str)?;
		let mut scope = json_schema::Scope::new();
		let schema = scope.compile_and_return(schema, false).unwrap();

		let paths = self.relevant_files(g)?;
		for path in paths.iter() {
			let data = std::fs::read_to_string(path.as_path())?;

			// Check that we can decode it.
			serde_yaml::from_str::<Member>(&data)
				.context(format!("checking {}", path.display()))?;

			// Check that it validates against the schema.
			Self::validate_schema(&schema, &data)?;
		}

		println!("Validated {} file{}.", paths.len(), crate::cmd::plural(paths.len()));

		Ok(())
	}

	fn relevant_files(&self, g: &GlobalConfig) -> Result<Vec<PathBuf>> {
		if let Some(files) = &self.files {
			Ok(files.clone())
		} else {
			let pattern = format!("{}/**/*.y*", g.members_dir.display());
			let mut files = vec![];

			for entry in glob(&pattern)? {
				files.push(entry?);
			}

			Ok(files)
		}
	}

	fn validate_schema<'a>(schema: &'a ScopedSchema<'a>, data: &str) -> Result<()> {
		let mut doc_as_yaml: serde_yaml::Value = serde_yaml::from_str(data)?;
		doc_as_yaml.apply_merge()?;

		let doc_as_json: serde_json::Value =
			serde_yaml::from_value(serde_yaml::to_value(&doc_as_yaml)?)?;

		let res = schema.validate(&doc_as_json);

		if !res.is_valid() || !res.is_strictly_valid() {
			anyhow::bail!("Validation failed for {:#?}", res);
		}

		Ok(())
	}
}
