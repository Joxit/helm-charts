use anyhow::{Context, Error, Result};
use convert_case::{Case, Casing};
use std::fs;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

pub struct Chart {
  pub name: String,
  pub pretty_name: String,
  pub app_version: String,
  pub yaml: Yaml,
}

fn pretty_name(name: &str) -> String {
  name.to_case(Case::Title).replace("Ui", "UI")
}

impl TryFrom<PathBuf> for Chart {
  type Error = Error;

  fn try_from(path: PathBuf) -> Result<Chart> {
    let s = fs::read_to_string(&path).with_context(|| format!("Failed to open {:?}", path))?;
    let docs = YamlLoader::load_from_str(&s)
      .with_context(|| format!("Failed to parse YAML file {:?}", path))?;
    let doc = &docs[0];

    let name = doc["name"]
      .as_str()
      .with_context(|| format!("Failed to found Chart name from {:?}", path))?;

    let app_version = doc["appVersion"]
      .as_str()
      .with_context(|| format!("Failed to found Chart app version from {:?}", path))?;

    Ok(Chart {
      name: name.to_string(),
      pretty_name: pretty_name(name),
      app_version: app_version.to_string(),
      yaml: doc.clone(),
    })
  }
}
