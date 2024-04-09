#![cfg(test)]

use anyhow::Error;
use std::process::ChildStderr;
use std::process::Stdio;
use std::io::Read;
use std::io::Write;
use ptyprocess::{errno::Errno, stream::Stream, PtyProcess};
use std::fs::File;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::BufRead;

#[tokio::test]
async fn t2() -> Result<(), Error> {
	let path = assert_cmd::cargo::cargo_bin("collective");
    let mut command = std::process::Command::new(&path);
    command.env("RUST_BACKTRACE", "1");
    command.env("TERM", "dumb");
    command.env("NO_COLOR", "1");
    command.arg("new");
    command.arg("evidence");
	command.arg("--cache");
	command.arg("off");
    command.arg("--config");
    let cfg_path = std::env!("CARGO_MANIFEST_DIR").to_string() + "/collective.yaml";
    command.arg(cfg_path);

	let proc = PtyProcess::spawn(command)?;
	let pty = proc.get_raw_handle()?;
	let mut writer = LineWriter::new(&pty);
    let mut reader = BufReader::new(&pty);

	let (tx, mut rx) = tokio::sync::mpsc::channel(16);
	tokio::spawn(async move {
		let pty = proc.get_raw_handle().unwrap();
		let mut buf = [0u8; 1024];
		loop {
			let s = reader.read(&mut buf).unwrap();
			if s == 0 {
				break;
			}
			let s = std::str::from_utf8(&buf[..s]).unwrap();
			tx.send(s.to_string()).await.unwrap();
		}
	});
    
    Ok(())
}

fn expect(p: &mut dyn BufRead, expected: &str) -> Result<(), Error> {
	let mut total = String::new();
    let mut buf = [0u8; 1024];
	loop {
		let s = p.read(&mut buf).unwrap();
		if s == 0 {
			break;
		}
		let s = std::str::from_utf8(&buf[..s]).unwrap();
		total.push_str(s);
		dbg!(&total);
	}

	if !total.contains(expected) {
		return anyhow::bail!("Expected: {}, got: {}", expected, total);
	}

	Ok(())
}
