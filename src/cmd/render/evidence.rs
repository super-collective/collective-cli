use crate::fellowship::FellowshipReport;

use sailfish::TemplateOnce;

#[derive(Debug, clap::Parser)]
pub struct RenderEvidenceCommand {}

impl RenderEvidenceCommand {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string("1.evidence").unwrap();
        let report: FellowshipReport = serde_yaml::from_str(&file).unwrap();

        let ctx = crate::template::EvidenceTemplate { report };
        println!("{}", ctx.render_once().unwrap());
        Ok(())
    }
}
