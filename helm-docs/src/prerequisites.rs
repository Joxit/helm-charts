use anyhow::{Context, Result};
use std::io::Write;

const PREREQUISITES: &str = r#"
  * **Helm 3.2+** (Helm 2 is not supported)
  * **Kubernetes 1.19+** - This is the earliest version of Kubernetes tested.
    It is possible that this chart works with earlier versions but it is untested.
"#;

pub fn generate_prerequisites<W: Write>(writer: &mut W) -> Result<()> {
  writeln!(writer, "{}", PREREQUISITES).with_context(|| format!("Failed to write file"))
}
