use tokio::process::Command;
use std::io;
use core::future::Future;
use std::process::Output;

static OUTPUT_DIR: &'static str = "/home/tom/Desktop/tmp";
static REPO_URL: &'static str =
  "https://github.com/tberghuis/watch-and-read-comments-for-youtube.git";

#[tokio::main]
async fn main() {
  // todo remove when finished
  Command::new("/bin/bash")
    .arg("-c")
    .arg("rm -rf *")
    .current_dir(OUTPUT_DIR)
    .output().await.expect("clean tmp dir failed");

  clone_master().await;

  let branches = get_branch_list().await;
  println!("{:?}", branches);

  for branch in branches.iter() {
    mkdir_branch(branch).await;
    clone_branch(branch).await;
  }
}

async fn clone_master() {
  command_wrapper(&format!("git clone {}", REPO_URL), OUTPUT_DIR).await.expect("clone master failed");
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

async fn get_branch_list() -> Vec<String> {
  println!("tmp_get_branch_list");
  let repo_dir = format!("{}/{}", OUTPUT_DIR, get_repo_name(REPO_URL));

  let output = Command::new("git")
    .arg("branch")
    .arg("-a")
    .current_dir(repo_dir)
    .output().await.unwrap();

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

  branches
}

async fn mkdir_branch(branch: &str) {
  let dir = format!("{}/branches/{}", OUTPUT_DIR, branch);

  Command::new("mkdir")
    .arg("-p")
    .arg(dir)
    .output().await.expect("TODO: panic message");
}

async fn clone_branch(branch: &str) {
  let dir = format!("{}/branches/{}", OUTPUT_DIR, branch);

  Command::new("git")
    .arg("clone")
    .arg("--depth")
    .arg("1")
    .arg("--branch")
    .arg(branch)
    .arg(REPO_URL)
    .arg(".")
    .current_dir(&dir)
    .output().await.expect("TODO: panic message");
}