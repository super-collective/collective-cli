// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

#![cfg(test)]

use glob::glob;
use std::process::Command;
use std::path::PathBuf;
use predicates::prelude::*;

#[test]
fn expect() {
	let pattern = format!("{}/**/*.expect", std::env!("CARGO_MANIFEST_DIR"));
	let files = glob(&pattern).unwrap();

	for file in files {
		// start expect process
		let file = file.unwrap();
		let mut cmd = Command::new("expect");
		cmd.current_dir(file.parent().unwrap());
		cmd.arg(&file);
		let output = cmd.output().unwrap();
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(output.status.success(), "{stderr}");

		let join_request_path = file.parent().unwrap().join("join_request").join("0.yaml");
		let evidence_path = file.parent().unwrap().join("evidence").join("0.evidence");

		assert!(join_request_path.exists() || evidence_path.exists());
		// delete folder
		let _ = std::fs::remove_dir_all(join_request_path.parent().unwrap());
		let _ = std::fs::remove_dir_all(evidence_path.parent().unwrap());
	}
}

#[test]
fn integration_example_fellowship() {
	let example_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
		.join("example")
		.join("fellowship");
	
	let mut cmd = assert_cmd::Command::cargo_bin("collective").unwrap();
	cmd.current_dir(example_dir);
	cmd.arg("check");
	cmd.arg("evidence")
		.assert()
		.stdout("Validated 2 files.\n");
}

#[test]
fn integration_example_potoc() {
	let example_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
		.join("example")
		.join("potoc");
	
	let mut cmd = assert_cmd::Command::cargo_bin("collective").unwrap();
	cmd.current_dir(example_dir);
	cmd.arg("check");
	cmd.arg("join-request")
		.assert()
		.stdout(predicate::str::contains("Validated 3 files."));
}
