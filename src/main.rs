extern crate clap;

use clap::*;
use std::process::Command;

fn main() {
  let matches = App::new("git stat")
    .subcommand(SubCommand::with_name("file"))
    .about("一周提交次数最多的文件列表")
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("file") {
    Command::new("git").arg("log").output().unwrap();
  }
}
