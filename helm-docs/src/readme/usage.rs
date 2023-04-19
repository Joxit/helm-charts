use crate::chart::Chart;
use anyhow::{Context, Result};
use std::io::Write;

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

pub fn generate_usage<W: Write>(writer: &mut W, chart: Chart) -> Result<()> {
  writeln!(
    writer,
    "{}",
    USAGE
      .replace("{chart-name}", &chart.name)
      .replace("{chart-pretty-name}", &chart.pretty_name)
  )
  .with_context(|| format!("Failed to write file"))
}
