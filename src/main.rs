mod cmd;
mod evidence;
mod prompt;
mod template;
mod traits;
mod member;
mod collective;

use clap::Parser;
use cmd::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

	Command::parse().run()
}
