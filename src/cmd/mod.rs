mod new;
mod render;

/// See out how Rust dependencies and features are enabled.
#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    #[clap(subcommand)]
    subcommand: SubCommand,

    #[clap(flatten)]
    global: GlobalArgs,
}

#[derive(Debug, clap::Parser)]
pub struct GlobalArgs {
    #[clap(long, short, global = true)]
    quiet: bool,
}

/// Sub-commands of the [Root](Command) command.
#[derive(Debug, clap::Subcommand)]
enum SubCommand {
    Create(new::NewCommand),
    Render(render::RenderCommand),
}

impl Command {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcommand {
            SubCommand::Create(c) => c.run(),
            SubCommand::Render(c) => c.run(),
        }
    }
}
