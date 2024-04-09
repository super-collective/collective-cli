mod check;
mod index;
mod new;
mod render;
mod schema;
mod tests;

use crate::config::{GlobalArgs, GlobalConfig};

pub const EVIDENCE_FOLDER: &str = "evidence";

/// See out how Rust dependencies and features are enabled.
#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
	#[clap(subcommand)]
	subcommand: SubCommand,

	/// The evidence folder.
	#[clap(flatten)]
	global: GlobalArgs,
}

#[derive(Debug, clap::Parser)]
pub struct OutputArgs {
	/// Write the output to a file.
	#[clap(long, short)]
	output: Option<std::path::PathBuf>,

	/// Write the output to the standard output.
	#[clap(long, short, conflicts_with = "output")]
	stdout: bool,
}

/// Sub-commands of the [Root](Command) command.
#[derive(Debug, clap::Subcommand)]
enum SubCommand {
	/// Create a new report.
	New(new::NewCommand),
	/// Render an evidence report.
	Render(render::RenderCommand),
	/// Generate a schema of a report.
	Schema(schema::SchemaCommand),
	/// Check something for validity.
	Check(check::CheckCommand),
	/// Index some files.
	Index(index::IndexCommand),
}

impl Command {
	pub fn run(self) -> anyhow::Result<()> {
		let g: GlobalConfig = self.global.try_into()?;

		match &self.subcommand {
			SubCommand::New(c) => c.run(&g),
			SubCommand::Render(c) => c.run(&g),
			SubCommand::Schema(c) => c.run(),
			SubCommand::Check(c) => c.run(&g),
			SubCommand::Index(c) => c.run(),
		}
	}
}

impl OutputArgs {
	pub fn write_schema(&self, default_path: &str, data: &str) -> anyhow::Result<()> {
		if let Some(path) = &self.output {
			std::fs::write(path, data)?;
			println!("Wrote schema to '{}'", path.display());
		} else if self.stdout {
			println!("{}", data);
		} else {
			let path = std::path::PathBuf::from(default_path);
			std::fs::write(&path, data)?;
			println!("Wrote schema to '{}'", path.display());
		}

		Ok(())
	}
}

pub fn plural(count: usize) -> &'static str {
	if count == 1 {
		""
	} else {
		"s"
	}
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum OnOff {
	On,
	Off,
}
