use std::io::{BufReader, BufRead};
use std::os::unix::process::parent_id;
use std::process::{Command, Stdio};

pub fn get_jobs() -> Option<String> {
    let ppid = parent_id();
    let mut cmd = Command::new("ps")
        .arg("--ppid")
        .arg(ppid.to_string())
        .arg("-oppid=")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    Some((stdout_reader.lines().count() - 1).to_string())
}
