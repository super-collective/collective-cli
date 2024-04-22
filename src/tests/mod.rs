#![cfg(test)]

use glob::glob;
use std::process::Command;

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
