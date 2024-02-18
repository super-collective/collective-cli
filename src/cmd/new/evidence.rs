#[derive(Debug, clap::Parser)]
pub struct NewEvidenceCommand {}

impl NewEvidenceCommand {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!("new");
    }
}
