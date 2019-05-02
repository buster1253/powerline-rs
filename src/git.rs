// read the .git/HEAD file and remove ref: refs/heads/ leaving only the branch
// if ref: is not present then assume it's a SHA-1 of the commit
// find the refs/* in packed-refs with the same hash
// if it's refs/tags/ then prepend a symbol to tell it's a tag

use std::process::{Command, Stdio};

pub fn get_branch() -> Option<String> {
    let cmd = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .expect("shit");

    if cmd.status.success() {
        let mut output = String::from_utf8(cmd.stdout).unwrap();
        output = output.replace("\n", "");
        return Some(output);
        //return Some(String::from_utf8(cmd.stdout.pop().unwrap()).unwrap());
    }
    else {
        return None
    }
    //println!("{}", output);
    //io::stdout().write_all(&cmd.stdout).unwrap();
}
