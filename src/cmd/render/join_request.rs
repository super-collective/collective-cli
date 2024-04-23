// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::types::prelude::*;
use anyhow::{bail, Context};
use sailfish::TemplateOnce;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct RenderJoinRequestCommand {
	#[clap(index = 1)]
	path: PathBuf,
}

impl RenderJoinRequestCommand {
	pub fn run(&self) -> anyhow::Result<()> {
		if !self.path.exists() {
			bail!("evidence file does not exist: {}", self.path.display());
		}
		let file = std::fs::read_to_string(self.path.as_path()).context("reading evidence file")?;
		let request: JoinRequest = serde_yaml::from_str(&file)?;

		let ctx = crate::template::JoinRequestTemplateHtml { request };
		let rendered = ctx.render_once()?;

		let output_path = self.path.with_extension("html");
		std::fs::write(&output_path, rendered)?;

		println!("Rendered join request to {}", output_path.display());
		println!(
			"Tip: You can render it into a PDF with `htmldoc --webpage -f {} {}`",
			output_path.with_extension("pdf").display(),
			output_path.display()
		);

		Ok(())
	}
}
