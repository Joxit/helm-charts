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

impl From<PathBuf> for Chart {
  fn from(s: PathBuf) -> Chart {
    let s = fs::read_to_string(s).unwrap();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];

    let name = doc["name"].as_str().unwrap();

    Chart {
      name: name.to_string(),
      pretty_name: pretty_name(name),
    }
  }
}
