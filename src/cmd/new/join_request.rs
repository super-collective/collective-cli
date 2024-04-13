use crate::{
	cmd::OnOff,
	config::{GlobalArgs, GlobalConfig},
	prompt::Prompt,
	types::join_request::JoinRequest,
};
use anyhow::{anyhow, Context};
use std::path::PathBuf;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, clap::Parser)]
pub struct NewJoinRequestCommand {
	#[clap(index = 1, default_value = "cli")]
	mode: GenerationMode,

	/// Whether to cache the answers.
	#[clap(long, default_value = "on")]
	cache: OnOff,

	#[clap(flatten)]
	global: GlobalArgs,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum GenerationMode {
	/// A template member request.
	Template,
	/// An example request.
	Example,
	/// CLI will prompt for the member request.
	Cli,
}

impl NewJoinRequestCommand {
	pub fn run(&self, g: &GlobalConfig) -> Result<()> {
		let data = match self.mode {
			//GenerationMode::Template => JoinRequest::template().into(),
			//GenerationMode::Example => MemberRequest::example().into(),
			GenerationMode::Cli => self.run_prompt(g)?,
			_ => todo!(),
		};

		let path = self.find_good_path(g)?;
		std::fs::write(&path, data)?;
		println!("🎉 Wrote partial member request to '{}'.", path.display());

		Ok(())
	}

	fn find_good_path(&self, g: &GlobalConfig) -> Result<PathBuf> {
		std::fs::create_dir_all(&g.join_requests_dir)
			.context("creating the join request directory")?;
		for i in 0..1000 {
			// lol shit code
			let path = g.join_requests_dir.join(i.to_string()).with_extension("yaml");

			if !path.exists() {
				return Ok(path);
			}
		}

		Err(anyhow!("Could not find a good path. Please use `--evidence` to specify an empty."))
	}

	fn run_prompt(&self, g: &GlobalConfig) -> Result<String> {
		let filled = self.query(g)?;

		println!("Please fill out the remaining TODOs");
		serde_yaml::to_string(&filled).map_err(Into::into)
	}

	fn query(&self, g: &GlobalConfig) -> Result<JoinRequest> {
		let mut prompt = Prompt::new(self.cache == OnOff::On)?;
		JoinRequest::query_with_id(&g.collective, &mut prompt)
	}
}
