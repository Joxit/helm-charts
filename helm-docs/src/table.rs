use regex::Regex;
use std::fs;
use std::path::PathBuf;

fn generate_key(key: &String, line: &str) -> String {
  for idx in 0..line.len() {
    if line.get(idx..idx + 1).unwrap() != " " {
      return key
        .split(".")
        .take(idx / 2)
        .collect::<Vec<&str>>()
        .join(".");
    }
  }
  String::new()
}

fn generate_default_value(line: &str) -> String {
  line.split_once(":").unwrap().1.trim().to_string()
}

pub fn generate_table(path: PathBuf) {
  let mut state = 0;
  let mut comment = String::new();
  let mut key = String::new();
  let key_regex = Regex::new("^\\s*[a-zA-Z0-9]").unwrap();
  for line in fs::read_to_string(path).unwrap().split("\n") {
    if line.trim().starts_with("## ") {
      println!("");
      println!("#{}", line);
      println!("");
      println!("| Value | Default | Description |");
      println!("| --- | --- | --- |");
      comment = String::new();
      state = 0;
    } else if key_regex.is_match(line) {
      if state == 0 {
        key = format!("{}", line.trim().split(":").next().unwrap())
      } else if state == 1 {
        key = format!(
          "{}.{}",
          generate_key(&key, &line),
          line.trim().split(":").next().unwrap()
        )
      }
      if comment.len() > 0 {
        println!(
          "| `{}` | `{}` | {} |",
          key,
          generate_default_value(line),
          comment
        );
      }
      comment = String::new();
    } else if line.trim().starts_with("# ") {
      state = 1;
      comment = format!("{}{}", comment, line.trim().trim_start_matches("#"))
        .trim()
        .to_string();
    } else if line.trim().len() > 0 {
      state = 2;
    }
  }
}
