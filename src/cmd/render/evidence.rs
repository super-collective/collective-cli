use crate::collective::fellowship::FellowshipEvidenceReport;
use anyhow::{bail, Context};
use sailfish::TemplateOnce;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct RenderEvidenceCommand {
	#[clap(index = 1)]
	path: PathBuf,
}

impl RenderEvidenceCommand {
	pub fn run(&self) -> anyhow::Result<()> {
		if !self.path.exists() {
			bail!("evidence file does not exist: {}", self.path.display());
		}
		let file = std::fs::read_to_string(self.path.as_path()).context("reading evidence file")?;
		let report: FellowshipEvidenceReport = serde_yaml::from_str(&file)?;

		let ctx = crate::template::EvidenceTemplate { report: todo!() };
		let rendered = ctx.render_once()?;

		let output_path = self.path.with_extension("html");
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
