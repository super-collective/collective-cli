mod cmd;
mod evidence;
mod fellowship;
mod prompt;
mod template;
mod traits;

use clap::Parser;
use cmd::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

	Command::parse().run()
}
