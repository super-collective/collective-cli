mod new;
mod render;
mod schema;

use std::path::PathBuf;

/// See out how Rust dependencies and features are enabled.
#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
	#[clap(subcommand)]
	subcommand: SubCommand,

	/// The collective to use.
	#[clap(long, short, default_value = "fellowship")]
	collective: crate::evidence::Collective,

	#[clap(flatten)]
	global: GlobalArgs,
}

#[derive(Debug, clap::Parser)]
pub struct GlobalArgs {
	#[clap(long, short, global = true)]
	quiet: bool,
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
}

impl Command {
	pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
		match &self.subcommand {
			SubCommand::New(c) => c.run(),
			SubCommand::Render(c) => c.run(),
			SubCommand::Schema(c) => c.run(),
		}
	}
}

impl OutputArgs {
	pub fn write_schema(&self, data: &str) -> Result<(), Box<dyn std::error::Error>> {
		if let Some(path) = &self.output {
			std::fs::write(path, data)?;
		} else {
			println!("{}", data);
		}

		Ok(())
	}

	pub fn write_evidence(&self, data: &str) -> Result<(), Box<dyn std::error::Error>> {
		if self.stdout {
			println!("{}", data);
		} else if let Some(path) = &self.output {
			std::fs::write(path, data)?;
			println!("Wrote evidence to {}", path.display());
		} else {
			let path = Self::find_good_path()?;
			assert!(!path.exists(), "Should not try to overwrite data");

			std::fs::write(&path, data)?;
			println!("Wrote evidence to {}", path.display());
		}

		Ok(())
	}

	fn find_good_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
		let date = chrono::Local::now().format("%Y-%m-%d").to_string();

		for i in 0..100 {
			let mut path = date.clone();
			if i != 0 {
				path.push_str(&format!(".{}", i));
			}
			path.push_str(".evidence");

			let path = PathBuf::from(path);
			if !path.exists() {
				return Ok(path);
			}
		}

		Err("Could not find a good path".into())
	}
}
