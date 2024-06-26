// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::EvidenceReport;
use anyhow::{bail, Context};
use sailfish::TemplateOnce;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct RenderEvidenceCommand {
	#[clap(index = 1)]
	path: PathBuf,

	/// Render in Markdown instead of HTML.
	#[clap(long, alias = "md")]
	pub markdown: bool,
}

impl RenderEvidenceCommand {
	pub fn run(&self) -> anyhow::Result<()> {
		if !self.path.exists() {
			bail!("evidence file does not exist: {}", self.path.display());
		}
		let file = std::fs::read_to_string(self.path.as_path()).context("reading evidence file")?;
		let report: EvidenceReport = serde_yaml::from_str(&file)?;

		let (rendered, ext) = if self.markdown {
			let ctx = crate::template::EvidenceMdTemplate { report };
			(ctx.render_once()?, "md")
		} else {
			let ctx = crate::template::EvidenceTemplate { report };
			(ctx.render_once()?, "html")
		};

		let output_path = self.path.with_extension(ext);
		std::fs::write(&output_path, rendered)?;

		println!("Rendered evidence report to {}", output_path.display());
		println!(
			"Tip: You can render it into a PDF with `htmldoc --webpage -f {} {}`",
			output_path.with_extension("pdf").display(),
			output_path.display()
		);

		Ok(())
	}
}
