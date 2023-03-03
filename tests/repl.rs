use std::{process::Command, thread::sleep, time::Duration};

/**
 * Testing running Ledger in REPL mode.
 */
use interactive_process::InteractiveProcess;

#[test]
fn run_repl() {
    let cmd = Command::new("ledger");

    let mut proc = InteractiveProcess::new(cmd, |line| {
        println!("Got: {}", line.unwrap());
    })
    .unwrap();

    proc.send("b master").unwrap();
    sleep(Duration::from_secs(1));
    
    sleep(Duration::from_millis(1));
    proc.close().kill().unwrap();
    
    assert!(false);
}