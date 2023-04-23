use self::prerequisites::generate_prerequisites;
use self::table::generate_table;
use self::usage::generate_usage;
use crate::chart::Chart;
use anyhow::Result;
use regex::{Captures, Match, Regex};
use std::io::Write;
use std::path::PathBuf;

mod prerequisites;
mod table;
mod usage;

pub struct Readme {
  chart: Chart,
  values: PathBuf,
}

impl Readme {
  pub fn new(directory: &PathBuf) -> Result<Self> {
    Ok(Self {
      chart: Chart::try_from(directory.join("Chart.yaml"))?,
      values: directory.join("values.yaml"),
    })
  }

  pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
    let template = DEFAULT_TEMPLATE.trim_end();
    let template_regex = Regex::new(r#"\{\{\s*([a-zA-Z0-9._]+)\s*\}\}"#).unwrap();

    for line in template.split("\n") {
      let line = template_regex
        .captures_iter(line)
        .try_fold(line.to_string(), |acc, elt| self.replace_tokens(acc, elt))?;
      writeln!(writer, "{}", line)?;
    }
    Ok(())
  }

  fn replace_tokens(&self, line: String, captures: Captures) -> Result<String> {
    if let Some(regex_match) = captures.get(1) {
      let raw_match = captures.get(0).unwrap().as_str();
      if let Some(content) = self.content_by_match(&regex_match) {
        return Ok(line.replace(raw_match, content?.as_str()).to_string());
      }
    }
    Ok(line)
  }

  fn content_by_match(&self, regex_match: &Match) -> Option<Result<String>> {
    match regex_match.as_str() {
      "prettyName" => Some(Ok(self.chart.pretty_name.clone())),
      "prerequisites" => Some(generate_prerequisites()),
      "usage" => Some(generate_usage(&self.chart)),
      "configuration" => Some(generate_table(&self.values)),
      _ => None,
    }
  }
}

const DEFAULT_TEMPLATE: &'static str = r#"# {{ prettyName }} Chart

## Prerequisites

{{ prerequisites }}

## Usage

{{ usage }}

## Configuration

{{ configuration }}
"#;
