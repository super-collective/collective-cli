use crate::fellowship::FellowshipReport;
use std::path::PathBuf;

use sailfish::TemplateOnce;

#[derive(Debug, clap::Parser)]
pub struct RenderEvidenceCommand {
	#[clap(index = 1)]
	path: PathBuf,
}

impl RenderEvidenceCommand {
	pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		let file = std::fs::read_to_string(self.path.as_path())?;
		let report: FellowshipReport = serde_yaml::from_str(&file)?;

		let ctx = crate::template::EvidenceTemplate { report };
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
