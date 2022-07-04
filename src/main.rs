use tokio::process::Command;
use std::io;
use core::future::Future;
use std::process::Output;
use std::env;
use std::process;
use std::path::PathBuf;


#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  if args.len() != 3 {
    println!("usage: checkout-all-branches-rust <repo_url> <output_dir>");
    println!("  e.g. checkout-all-branches-rust https://github.com/user/repo.git .");
    process::exit(1);
  }

  let output_dir = &args[2];
  let repo_url = &args[1];
  let repo_name = get_repo_name(repo_url);

  check_output_dir_empty(output_dir);

  clone_master(output_dir, repo_url).await;

  let branches = get_branch_list(output_dir, repo_name).await;
  println!("{:?}", branches);

  for branch in branches.iter() {
    mkdir_branch(output_dir, branch).await;
    clone_branch(output_dir, repo_url, branch).await;
  }
}

fn check_output_dir_empty(output_dir: &str) {
  let path = PathBuf::from(output_dir);
  let is_empty = path.read_dir().unwrap().next().is_none();
  if !is_empty {
    println!("output_dir is not empty, exiting...");
    process::exit(1);
  }
}


async fn clone_master(output_dir: &str, repo_url: &str) {
  command_wrapper(&format!("git clone {}", repo_url), output_dir).await.expect("clone master failed");
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

async fn get_branch_list(output_dir: &str, repo_name: &str) -> Vec<String> {
  println!("tmp_get_branch_list");
  let repo_dir = format!("{}/{}", output_dir, repo_name);

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

async fn mkdir_branch(output_dir: &str, branch: &str) {
  let dir = format!("{}/branches/{}", output_dir, branch);

  Command::new("mkdir")
    .arg("-p")
    .arg(dir)
    .output().await.expect("TODO: panic message");
}

async fn clone_branch(output_dir: &str, repo_url: &str, branch: &str) {
  let dir = format!("{}/branches/{}", output_dir, branch);

  Command::new("git")
    .arg("clone")
    .arg("--depth")
    .arg("1")
    .arg("--branch")
    .arg(branch)
    .arg(repo_url)
    .arg(".")
    .current_dir(&dir)
    .output().await.expect("TODO: panic message");
}