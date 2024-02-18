use crate::cmd::OutputArgs;
use crate::fellowship::FellowshipReport;
use std::path::PathBuf;

use sailfish::TemplateOnce;

#[derive(Debug, clap::Parser)]
pub struct RenderEvidenceCommand {
    #[clap(index = 1)]
    path: PathBuf,

    #[clap(flatten)]
    output: OutputArgs,
}

impl RenderEvidenceCommand {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(self.path.as_path())?;
        let report: FellowshipReport = serde_yaml::from_str(&file)?;

        let ctx = crate::template::EvidenceTemplate { report };
        let rendered = ctx.render_once()?;

        self.output.write(&rendered)
    }
}
