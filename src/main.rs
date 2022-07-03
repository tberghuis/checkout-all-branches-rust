use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

static OUTPUT_DIR: &'static str = "/home/tom/Desktop/tmp";
static REPO_URL: &'static str =
  "https://github.com/tberghuis/watch-and-read-comments-for-youtube.git";

fn main() -> io::Result<()> {
  let mut child_shell = Command::new("/bin/bash")
    .stdin(Stdio::piped())
    // .stdout(Stdio::piped())
    .spawn()
    .unwrap();

  let child_in = child_shell.stdin.as_mut().unwrap();


  let cmd = format!("cd {}\n", OUTPUT_DIR);
  child_in.write_all(cmd.as_bytes())?;
  let cmd = format!("git clone {}\n", REPO_URL);
  child_in.write_all(cmd.as_bytes())?;

  child_shell.wait_with_output();
  Ok(())
}
