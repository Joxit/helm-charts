use self::prerequisites::generate_prerequisites;
use self::table::generate_table;
use self::usage::generate_usage;
use crate::chart::Chart;
use anyhow::{Context, Result};
use regex::{Captures, Match, Regex};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

mod prerequisites;
mod table;
mod usage;

pub struct Readme {
  chart: Chart,
  values: PathBuf,
  directory: PathBuf,
}

impl Readme {
  pub fn new(directory: &PathBuf) -> Result<Self> {
    Ok(Self {
      chart: Chart::try_from(directory.join("Chart.yaml"))?,
      values: directory.join("values.yaml"),
      directory: directory.clone(),
    })
  }

  pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
    let template = self.get_template_content()?;
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
      "name" => Some(Ok(self.chart.name.clone())),
      "prettyName" => Some(Ok(self.chart.pretty_name.clone())),
      "appVersion" => Some(Ok(self.chart.app_version.clone())),
      "prerequisites" => Some(generate_prerequisites()),
      "usage" => Some(generate_usage(&self.chart)),
      "configuration" => Some(generate_table(&self.values)),
      _ => None,
    }
  }

  fn get_template_content(&self) -> Result<String> {
    let path = self.directory.join("README.tmpl");
    let content = if path.exists() {
      fs::read_to_string(&path).with_context(|| format!("Failed to open {:?}", path))?
    } else {
      DEFAULT_TEMPLATE.trim_end().to_string()
    };
    Ok(content.trim_matches('\n').to_string())
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
