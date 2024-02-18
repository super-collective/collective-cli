mod new;
mod render;
mod schema;

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
    #[clap(long, short)]
    output: Option<std::path::PathBuf>,
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
    pub fn write(&self, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(output) = &self.output {
            std::fs::write(output, data)?;
        } else {
            println!("{}", data);
        }

        Ok(())
    }
}
