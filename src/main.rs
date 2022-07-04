use tokio::process::Command;
use std::io;
use core::future::Future;
use std::process::Output;

static OUTPUT_DIR: &'static str = "/home/tom/Desktop/tmp";
static REPO_URL: &'static str =
  "https://github.com/tberghuis/watch-and-read-comments-for-youtube.git";


#[tokio::main]
async fn main_tmp() -> Result<(), Box<dyn std::error::Error>> {
  // todo remove when finished
  Command::new("/bin/bash")
    .arg("-c")
    .arg("rm -rf *")
    .current_dir(OUTPUT_DIR)
    .output().await?;

  let output = command_wrapper(&format!("git clone {}", REPO_URL), OUTPUT_DIR).await?;

  // fuck this was hard
  println!("stdout {}", std::str::from_utf8(&output.stdout).unwrap());
  println!("stderr {}", std::str::from_utf8(&output.stderr).unwrap());

  Ok(())
}


fn command_wrapper(command: &str, directory: &str) -> impl Future<Output=io::Result<Output>> {
  let v: Vec<_> = command.split(' ').collect();
  Command::new(v[0])
    .args(&v[1..])
    .current_dir(directory)
    .output()
}

fn get_repo_name(url: &str) -> &str {
  let v: Vec<&str> = url.split('/').collect();
  let basename = v[v.len() - 1];
  let repo_name = basename.split('.').collect::<Vec<&str>>()[0];
  repo_name
}

#[tokio::main]
async fn main() {
  tmp_get_branch_list().await;
}

// i don't understand this box dyn error shit
async fn tmp_get_branch_list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
  println!("tmp_get_branch_list");

  let output = Command::new("git")
    .arg("branch")
    .arg("-a")
    .current_dir("/home/tom/Desktop/tmp/watch-and-read-comments-for-youtube")
    .output().await?;

  let output_lines = std::str::from_utf8(&output.stdout).unwrap();

  println!("{}", output_lines);

  let branches: Vec<String> = output_lines.split("\n")
    .filter(|&s| !s.contains("*"))
    .filter(|&s| !s.contains("->"))
    .filter(|&s| !s.is_empty())
    .map(|s| s.trim())
    .map(|s| s.replace("remotes/origin/", ""))
    .collect();

  println!("{}", branches[0]);
  println!("{}", branches.len());
  // println!("stderr: {}", output.stderr);

  Ok(branches)
}
