use crate::collective::CollectiveId;
use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

type Result<T> = anyhow::Result<T>;

#[derive(Debug, clap::Parser)]
pub struct GlobalArgs {
	/// The root folder of the collective.
	#[clap(long, global = true)]
	root: Option<PathBuf>,

	#[clap(long, global = true, default_value = "collective.yaml")]
	config: PathBuf,

	#[clap(long, global = true)]
	collective: Option<CollectiveId>,

	#[clap(long, global = true)]
	members_dir: Option<PathBuf>,

	#[clap(long, global = true)]
	join_requests_dir: Option<PathBuf>,

	#[clap(long, global = true)]
	evidence_dir: Option<PathBuf>,

	#[clap(long, short, global = true)]
	quiet: bool,
}

#[derive(Debug)]
pub struct GlobalConfig {
	pub collective: CollectiveId,
	pub members_dir: PathBuf,
	pub evidence_dir: PathBuf,
	pub join_requests_dir: PathBuf,
	pub config_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigFile {
	collective: Option<CollectiveId>,
	members_dir: Option<PathBuf>,
	join_requests_dir: Option<PathBuf>,
	evidence_dir: Option<PathBuf>,
}

impl TryFrom<GlobalArgs> for GlobalConfig {
	type Error = anyhow::Error;

	fn try_from(args: GlobalArgs) -> Result<Self> {
		let file = std::fs::read_to_string(&args.config)
			.with_context(|| format!("reading config file: {}", &args.config.display()))?;
		let cfg: ConfigFile = serde_yaml::from_str(&file).context("parsing config file")?;
		log::info!("Read config file from: {}", args.config.display());

		let Some(collective) = args.collective.or(cfg.collective) else {
			bail!("The collective to use was not specified in the config file nor as CLI flag. Please either add a `collective` entry to the config file, or pass `--collective to the command.");
		};
		let r = Self {
			collective,
			members_dir: args.members_dir.or(cfg.members_dir).unwrap_or("members".into()),
			join_requests_dir: args
				.join_requests_dir
				.or(cfg.join_requests_dir)
				.unwrap_or("join_requests".into()),
			evidence_dir: args.evidence_dir.or(cfg.evidence_dir).unwrap_or("evidence".into()),
			config_path: args.config,
		};
		log::info!("{:?}", &r);
		Ok(r)
	}
}
