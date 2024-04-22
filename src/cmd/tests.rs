#![cfg(test)]

use assert_cmd::Command;
use assert_fs::fixture::*;
use std::path::PathBuf;

#[test]
fn schema_generator_works() {
	let temp = assert_fs::TempDir::new().unwrap();
	let cfg_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
		.join("src")
		.join("tests")
		.join("collective.yaml");

	let mut cmd = Command::cargo_bin("collective").unwrap();
	cmd.current_dir(temp.path());
	cmd.arg("--config").arg(cfg_path);
	cmd.arg("schema");
	cmd.arg("evidence").assert().success();

	let schema = temp.child("evidence.schema.json");
	assert!(schema.exists());
}

#[test]
fn check_evidence_works() {
	let temp = assert_fs::TempDir::new().unwrap();
	let cfg_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
		.join("src")
		.join("tests")
		.join("collective.yaml");

	let mut cmd = Command::cargo_bin("collective").unwrap();
	cmd.current_dir(temp.path())
		.arg("--config")
		.arg(cfg_path)
		.arg("check")
		.arg("evidence")
		.assert()
		.success()
		.stdout("Validated 0 files.\n");
}

#[test]
fn example_works() {
	let temp = assert_fs::TempDir::new().unwrap();
	let cfg_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
		.join("src")
		.join("tests")
		.join("collective.yaml");
	let evidence = temp.child("evidence");
	evidence.create_dir_all().unwrap();

	let mut cmd = Command::cargo_bin("collective").unwrap();
	cmd.current_dir(temp.path())
		.arg("--config")
		.arg(&cfg_path)
		.arg("--evidence-dir")
		.arg(evidence.path())
		.arg("new")
		.arg("evidence")
		.arg("example")
		.assert()
		.success()
		.stdout("🎉 Wrote partial evidence report to 'evidence/0.evidence'.\n");

	assert!(evidence.child("0.evidence").exists());

	// Check that we can validate it
	let mut cmd = Command::cargo_bin("collective").unwrap();
	cmd.current_dir(temp.path())
		.arg("--config")
		.arg(&cfg_path)
		.arg("--evidence-dir")
		.arg(evidence.path())
		.arg("check")
		.arg("evidence")
		.assert()
		.success()
		.stdout("Validated 1 file.\n");

	// now create indices
	let mut cmd = Command::cargo_bin("collective").unwrap();
	cmd.current_dir(temp.path())
		.arg("--config")
		.arg(&cfg_path)
		.arg("--evidence-dir")
		.arg(evidence.path())
		.arg("index")
		.arg("evidence")
		.assert()
		.success()
		.stdout("Scanned 2 and created 2 new links.\n");

	let date_link = evidence
		.child("by_date")
		.child("2024")
		.child("03")
		.child("20")
		.child("0.evidence");
	let reporter = evidence.child("by_reporter").child("max-power").child("0.evidence");

	assert!(date_link.exists());
	assert!(reporter.exists());

	// check the content of the links
	assert!(std::fs::symlink_metadata(date_link.path()).unwrap().file_type().is_symlink());
	assert!(std::fs::symlink_metadata(reporter.path()).unwrap().file_type().is_symlink());
}
