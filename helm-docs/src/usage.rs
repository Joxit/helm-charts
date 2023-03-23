use convert_case::{Case, Casing};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use yaml_rust::{YamlEmitter, YamlLoader};

const USAGE: &str = r#"
1. Add my Helm repository (named `joxit`)
```
helm repo add joxit https://helm.joxit.dev
```
2. Ensure you have access to the Helm chart and you see the latest chart version listed. If you have previously added the Helm repository, run `helm repo update`.
```
helm search repo joxit/{chart-name}
```
3. Now you're ready to install the {chart-pretty-name}! To install {chart-pretty-name} with the default configuration using Helm 3.2 run the following command below. This will deploy the {chart-pretty-name} on the default namespace.
```
helm upgrade --install {chart-name} joxit/{chart-name}
```
"#;

fn pretty_name(name: &str) -> String {
  name.to_case(Case::Title).replace("Ui", "UI")
}

pub fn generate_usage(path: PathBuf) {
  let s = fs::read_to_string(path).unwrap();
  let docs = YamlLoader::load_from_str(&s).unwrap();
  let doc = &docs[0];

  let name = doc["name"].as_str().unwrap();

  println!(
    "{}",
    USAGE
      .replace("{chart-name}", name)
      .replace("{chart-pretty-name}", pretty_name(name).as_str())
  )
}
