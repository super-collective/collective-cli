use crate::cmd::OutputArgs;
use crate::fellowship::FellowshipReport;

#[derive(Debug, clap::Parser)]
pub struct NewEvidenceCommand {
    #[clap(index = 1, default_value = "template")]
    mode: GenerationMode,

    #[clap(flatten)]
    output: OutputArgs,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum GenerationMode {
    /// A template evidence.
    Template,
    /// An example evidence.
    Example,
    // TODO:
    // CLI will prompt for the evidence.
    //Interactive,
}

impl NewEvidenceCommand {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = match self.mode {
            GenerationMode::Template => FellowshipReport::template(),
            GenerationMode::Example => FellowshipReport::example(),
        };

        self.output.write(data)
    }
}
