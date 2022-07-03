use tokio::process::Command;
use std::io;
use core::future::Future;
use std::process::Output;

static OUTPUT_DIR: &'static str = "/home/tom/Desktop/tmp";
static REPO_URL: &'static str =
  "https://github.com/tberghuis/watch-and-read-comments-for-youtube.git";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  // todo write a wrapper fn
  // let output = Command::new("git").arg("clone")
  //   .arg("--progress")
  //   .arg(REPO_URL)
  //   .current_dir(OUTPUT_DIR)
  //   .output();

  let output = command_wrapper(format!("git clone {}", REPO_URL), OUTPUT_DIR.to_string());

  println!("before await");

  let output = output.await?;

  // fuck this was hard
  println!("stdout {}", std::str::from_utf8(&output.stdout).unwrap());
  println!("stderr {}", std::str::from_utf8(&output.stderr).unwrap());

  println!("after last println");

  Ok(())
}


fn command_wrapper(command: String, directory: String) -> impl Future<Output=io::Result<Output>> {
  let v: Vec<_> = command.split(' ').collect();
  Command::new(v[0])
    .args(&v[1..])
    .current_dir(directory)
    .output()
}