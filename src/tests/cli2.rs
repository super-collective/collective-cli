#![cfg(test)]

use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use std::process::Stdio;
use anyhow::Error;
use tokio::process::ChildStderr;
use tokio::process::ChildStdout;
use tokio_stream::{StreamExt, StreamMap, Stream};
use tokio::sync::mpsc::Receiver;
use clap::Parser;
use std::io::Read;
use std::io::Write;
use expectrl::{
    interact::actions::lookup::Lookup, spawn, stream::stdin::Stdin, ControlCode,
    Regex,
};
use expectrl::process::unix::PtyStream;
use expectrl::process::unix::UnixProcess;
use expectrl::Session;
use expectrl::Expect;
use std::io::stdout;

#[test]
fn t2() -> Result<(), Error> {
    let path = assert_cmd::cargo::cargo_bin("collective");
    let mut command = path.display().to_string();
    command += " new";
    command += " join-request";
    command += " --cache off";
    command += " --config";
    let cfg_path = std::env!("CARGO_MANIFEST_DIR").to_string() + "/collective.yaml";
    command += &(" ".to_owned() + &cfg_path);
    let mut p = expectrl::spawn(&command).unwrap();

    p.expect(Regex(r".*move.*"))?;
    p.expect(Regex(r".+"))?;
    send_line(&mut p, "Fellowship")?;

    expect(&mut p, "Please enter legal name or pseudonym")?;
    send_line(&mut p, "asdf")?;

    expect(&mut p, "Please enter your Polkadot address")?;
    send_line(&mut p, "123")?;

    expect(&mut p, "Please enter your GitHub handle")?;
    send_line(&mut p, "ggwpez")?;

    expect(&mut p, "Rank to join")?;
    send_line(&mut p, "test")?;

    expect(&mut p, "date");
    send_line(&mut p, "")?;

    // date
    // title evidence
    expect(&mut p, "evidence title");
    // category DevelopmentRuntime
    // task title
    // task link
    // n
    // n
    
    Ok(())
}

fn send_line(p: &mut Session<UnixProcess, PtyStream>, input: &str) -> Result<(), Error> {
    for c in input.chars() {
        send_char(p, c)?;
    }
    send_char(p, '\r')?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    p.send_line(&"")?;
    std::thread::sleep(std::time::Duration::from_millis(10));

    Ok(())
}

fn send_char(p: &mut Session<UnixProcess, PtyStream>, input: char) -> Result<(), Error> {
    p.send(&input.to_string())?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    p.expect(Regex(r".+"))?;
    std::thread::sleep(std::time::Duration::from_millis(10));

    Ok(())
}

fn expect(p: &mut Session<UnixProcess, PtyStream>, expected: &str) -> Result<(), Error> {
    let got = p.expect(Regex(r".+"))?;
    std::thread::sleep(std::time::Duration::from_millis(10));

    let s = String::from_utf8_lossy(got.get(0).unwrap());
    assert!(s.contains(expected), "Expected: {:?}, Got: {:?}", expected, s);
    std::thread::sleep(std::time::Duration::from_millis(10));

    Ok(())
}
