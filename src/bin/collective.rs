use backtrace::Backtrace;
use clap::Parser;
use collective::cmd::Command;
use urlencoding::encode as url_escape;

fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");
	set_panic_hook();
	env_logger::init();

	// Handle the error so it wont print a stack trace.
	if let Err(e) = Command::parse().run() {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}

fn set_panic_hook() {
	std::panic::set_hook(Box::new(|panic_info| {
		let bt = Backtrace::new();

		let title = match panic_info.location() {
			Some(location) => format!("file '{}' at line {}", location.file(), location.line()),
			None => "<no location>".to_string(),
		};

		let cmd = std::env::args().collect::<Vec<String>>().join(" ");
		let trace = format!("PANIC: {:?}\nBACKTRACE:\n{:?}\nCMD:{}", panic_info, bt, cmd);

		let body = format!(
			"`v{}`: `{cmd}`\n```pre\n{:#?}\n```",
			std::env!("CARGO_PKG_VERSION"),
			panic_info
		);
		let title = format!("[cli] panic at {title}");
		let url = format!(
			"https://github.com/super-collective/collective-cli/issues/new?title={}&body={}",
			url_escape(&title),
			url_escape(&body)
		);
		eprintln!(
			"The program crashed. Please report this issue and include the 'panic.log' file: {:?}",
			url
		);

		if let Err(err) = std::fs::write("panic.log", &trace) {
			eprintln!(
				"The program crashed and could not write the panic log: {:?}\nwrite error: {:?}",
				trace, err
			);
		}
	}));
}
