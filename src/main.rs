use std::process::Command;

static OUTPUT_DIR: &'static str = "/home/tom/Desktop/tmp";
static REPO_URL: &'static str =
    "https://github.com/tberghuis/watch-and-read-comments-for-youtube.git";

fn main() {
    Command::new("cd").arg(OUTPUT_DIR).spawn();

    Command::new("git").arg("clone").arg(REPO_URL).spawn();
}
