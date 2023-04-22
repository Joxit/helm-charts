use anyhow::Result;

const PREREQUISITES: &str = r#"
  * **Helm 3.2+** (Helm 2 is not supported)
  * **Kubernetes 1.19+** - This is the earliest version of Kubernetes tested.
    It is possible that this chart works with earlier versions but it is untested.
"#;

pub fn generate_prerequisites() -> Result<String> {
  Ok(PREREQUISITES.trim_matches('\n').to_string())
}
