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

#[test]
fn t2() -> Result<(), Error> {
    crossterm::terminal::enable_raw_mode().unwrap();

    let path = assert_cmd::cargo::cargo_bin("collective");
    let mut command = std::process::Command::new(&path);
    command.env("RUST_BACKTRACE", "1");
    command.env("TERM", "dumb");
    command.env("NO_COLOR", "1");
    command.arg("new");
    command.arg("join-request");
    command.arg("--config");
    let cfg_path = std::env!("CARGO_MANIFEST_DIR").to_string() + "/collective.yaml";
    command.arg(cfg_path);
    
    let mut child = command
        .stdin(Stdio::piped())
        //.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    
    let mut stderr = child.stderr.take().unwrap();
    let mut stdin = child.stdin.take().unwrap();

    stdin.write_all(b"F").unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100));
    stdin.write_all(b"e").unwrap();

    let mut buf = [0u8; 1024];
    let s = stderr.read(&mut buf).unwrap();
    let s = std::str::from_utf8(&buf[..s]).unwrap();

    child.wait().unwrap();

    Ok(())
}

#[tokio::test]
async fn t1() -> Result<(), Error> {
    crossterm::terminal::enable_raw_mode().unwrap();
    // setup mpsc channels
   let (tx, mut rx) = tokio::sync::mpsc::channel(16);
   let tx2 = tx.clone();

    let path = assert_cmd::cargo::cargo_bin("collective");
    let mut command = Command::new(&path);
    command.env("RUST_BACKTRACE", "1");
    command.env("TERM", "");
    command.arg("new");
    command.arg("join-request");
    command.arg("--config");
    let cfg_path = std::env!("CARGO_MANIFEST_DIR").to_string() + "/collective.yaml";
    command.arg(cfg_path);
    
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    // copy stdout to stdout
    tokio::spawn(async move {
        // copy everything into the tx channel
        let mut buf = [0u8; 1024];
        loop {
            let s = stdout.read(&mut buf).await.unwrap();
            if s == 0 {
                break;
            }
            tx.send(buf[..s].to_vec()).await.unwrap();
        }
    });
    // same for stderr
    tokio::spawn(async move {
        // copy everything into the tx channel
        let mut buf = [0u8; 1024];
        loop {
            let s = stderr.read(&mut buf).await.unwrap();
            if s == 0 {
                break;
            }
            tx2.send(buf[..s].to_vec()).await.unwrap();
        }
    });

    // start polling the child task
    let mut done = tokio::spawn(async move {
        assert!(child.wait().await.unwrap().success(), "Child failed");
    });
    
    expect_output(&mut rx, "to move, enter to select, type to filter").await;
    
    stdin.write_all(b"F").await.unwrap();
    stdin.write_all(b"F").await.unwrap();

    expect_output(&mut rx, "legal name").await;

    return Ok(());

    // then drop when finished
    /*stdin.write_all(b"Fellowship\n").await.unwrap();
    stdin.write_all(b"Fellowship\n").await.unwrap();
    stdin.write_all(b"Fellowship\n").await.unwrap();

    expect_output(&mut stderr, "legal name").await;

    */

    Ok(())
}

async fn expect_output(child: &mut Receiver<Vec<u8>>, expected: &str) {
    let mut output = String::new();

    loop {
       tokio::select! {
            Some(buf) = child.recv() => {
                let s = std::str::from_utf8(&buf).unwrap();
                output.push_str(s);

                if output.contains(expected) {
                    return;
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
                assert!(false, "Timeout: {:?} while waiting for {:?}", output, expected);
            }
        }
    }

    assert!(output.contains(expected));
}
