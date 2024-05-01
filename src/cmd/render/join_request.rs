// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::*;
use anyhow::{bail, Context};
use sailfish::TemplateOnce;
use std::path::{Path, PathBuf};

#[derive(Debug, clap::Parser)]
pub struct RenderJoinRequestCommand {
	#[clap(index = 1)]
	path: Vec<PathBuf>,

	/// Also render to PDF. Requires `htmldoc` to be installed.
	#[clap(long)]
	pdf: bool,
}

impl RenderJoinRequestCommand {
	pub fn run(&self) -> anyhow::Result<()> {
		for path in self.path.iter() {
			self.run_with_path(path.clone())?;
		}

		Ok(())
	}

	fn run_with_path(&self, path: PathBuf) -> anyhow::Result<()> {
		if !path.exists() {
			bail!("evidence file does not exist: {}", path.display());
		}
		let file = std::fs::read_to_string(path.as_path()).context("reading evidence file")?;
		let request: JoinRequest =
			serde_yaml::from_str(&file).with_context(|| format!("parsing {:?}", path))?;

		let ctx = crate::template::JoinRequestTemplateHtml { request };
		let rendered = ctx.render_once()?;

		let html_path = path.with_extension("html");
		std::fs::write(&html_path, rendered)?;

		println!("Rendered to {}", html_path.display());

		if !self.pdf {
			println!(
				"Tip: You can render it into a PDF with `htmldoc --webpage -f {} {}`",
				html_path.with_extension("pdf").display(),
				html_path.display()
			);
		} else {
			Self::render_pdf(&html_path)?;
		}

		Ok(())
	}

	fn render_pdf(html_path: &Path) -> anyhow::Result<()> {
		let pdf_path = html_path.with_extension("pdf");
		let status = std::process::Command::new("htmldoc")
			.args(["--webpage", "-f", pdf_path.to_str().unwrap(), html_path.to_str().unwrap()])
			.stdout(std::process::Stdio::piped())
			.stderr(std::process::Stdio::piped())
			.status()
			.context("running htmldoc")?;

		if !status.success() {
			bail!("htmldoc failed");
		}

		println!("Rendered to {}", pdf_path.display());

		Ok(())
	}
}
