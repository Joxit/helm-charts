use anyhow::{Context, Error, Result};
use convert_case::{Case, Casing};
use std::fs;
use std::path::PathBuf;
use yaml_rust::YamlLoader;

pub struct Chart {
  pub name: String,
  pub pretty_name: String,
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

    Ok(Chart {
      name: name.to_string(),
      pretty_name: pretty_name(name),
    })
  }
}
