extern crate clap;
extern crate regex;

use clap::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::str;
use std::string::String;

type Record = HashMap<String, i32>;

/// https://stackoverflow.com/questions/41013941/finding-the-most-frequent-committer-to-a-specific-file
///
fn main() {
  let matches = App::new("git stat")
    .subcommand(
      SubCommand::with_name("file")
        .about("一周改动最多的文件列表")
        .arg(Arg::with_name("author").index(1).help("文件作者")),
    )
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("file") {
    // git log --since=1.weeks --author={author}
    let output = Command::new("git")
      .arg("log")
      .arg(format!(
        "--author={}",
        matches.value_of("author").expect("author无效")
      ))
      .arg("--since=1.weeks")
      .arg("--stat")
      .arg("--color")
      .output()
      .unwrap();

    // io::stdout().write_all(&output.stdout).unwrap();
    let output = str::from_utf8(&output.stdout).unwrap();

    // iterate output
    let re = Regex::new(r"([^\s]+)\s+(\|)\s+(\d+)").unwrap();
    let mut map: Record = HashMap::new();

    for cap in re.captures_iter(output) {
      let key = &cap[1];
      let count: i32 = cap[3].parse().unwrap();
      // 已经记录过
      if map.contains_key(key) {
        map.insert(key.to_string(), map.get(key).unwrap() + count);
      } else {
        map.insert(key.to_string(), count);
      }
    }

    let mut keys: Vec<String> = map.keys().map(|v| String::from(v)).collect();

    keys.sort_by(|a, b| map.get(b).cmp(&map.get(a)));

    for key in keys {
      println!("{} {:?}", key, map.get(&key).unwrap());
    }
  }
}
