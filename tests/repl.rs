/*
 * Testing running Ledger in REPL mode.
 */

/*
use std::{process::Command, thread::sleep, time::Duration};
use interactive_process::InteractiveProcess;

#[test]
fn run_repl() {
    let mut cmd = Command::new("ledger");
    cmd.args(["-f", "tests/journal.ledger"]);

    let mut proc = InteractiveProcess::new(cmd, |line| {
        let text = line.unwrap();
        if !text.starts_with(']') {
            return;
        }

        println!("Got: {}", text);
    })
    .unwrap();
    // sleep(Duration::from_secs(1));

    proc.send("b cash").unwrap();
    sleep(Duration::from_secs(1));

    // sleep(Duration::from_millis(1));
    // proc.close().kill().unwrap();

    // Close by issuing a command.
    proc.send("quit").unwrap();
    proc.wait().unwrap();
    //sleep(Duration::from_millis(1));

    assert!(true);
}
*/