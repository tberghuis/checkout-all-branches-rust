use tokio::process::Command;

static OUTPUT_DIR: &'static str = "/home/tom/Desktop/tmp";
static REPO_URL: &'static str =
  "https://github.com/tberghuis/watch-and-read-comments-for-youtube.git";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  // let output = Command::new("ls")
  // .current_dir(OUTPUT_DIR)
  // .output();


  let output = Command::new("git").arg("clone")
    .arg("--progress")
    .arg(REPO_URL)
    // .arg("2>&1")
    .current_dir(OUTPUT_DIR)
    .output();

  println!("before await");

  let output = output.await?;

  // assert!(output.status.success());
  // assert_eq!(output.stdout, b"hello world\n");

  // fuck this was hard
  println!("stdout {}", std::str::from_utf8(&output.stdout).unwrap());
  println!("stderr {}", std::str::from_utf8(&output.stderr).unwrap());

  

  println!("after last println");

  Ok(())
}