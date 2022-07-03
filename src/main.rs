use tokio::process::Command;

static OUTPUT_DIR: &'static str = "/home/tom/Desktop/tmp";
static REPO_URL: &'static str =
  "https://github.com/tberghuis/watch-and-read-comments-for-youtube.git";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // let output = Command::new("ls")
  //   .arg("-la")
  //   .current_dir(OUTPUT_DIR)
  //   .output();

  let output = Command::new("git").arg("clone")
    .arg("--progress")
    .arg(REPO_URL)
    .current_dir(OUTPUT_DIR)
    .output();

  println!("before await");

  let output = output.await?;

  // fuck this was hard
  println!("stdout {}", std::str::from_utf8(&output.stdout).unwrap());
  println!("stderr {}", std::str::from_utf8(&output.stderr).unwrap());

  println!("after last println");

  Ok(())
}