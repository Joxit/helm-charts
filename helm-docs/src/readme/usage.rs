use crate::chart::Chart;
use anyhow::Result;

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

pub fn generate_usage(chart: &Chart) -> Result<String> {
  Ok(
    USAGE
      .replace("{chart-name}", &chart.name)
      .replace("{chart-pretty-name}", &chart.pretty_name)
      .trim_matches('\n')
      .to_string(),
  )
}
