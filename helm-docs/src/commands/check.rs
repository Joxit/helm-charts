use crate::chart::Chart;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;
use yaml_rust::YamlLoader;

#[derive(Debug, StructOpt)]
pub struct Check {
  #[structopt(required = true)]
  directories: Vec<PathBuf>,
}

impl Check {
  pub fn exec(&self) -> Result<()> {
    for directory in &self.directories {
      self.check_chart(&directory)?;
    }
    Ok(())
  }

  fn check_chart(&self, directory: &PathBuf) -> Result<()> {
    let chart = Chart::try_from(directory.join("Chart.yaml"))?;
    let values = directory.join("values.yaml");

    let image = self.check_chart_file_image(&chart, directory)?;
    Ok(())
  }

  fn check_chart_file_image(&self, chart: &Chart, directory: &PathBuf) -> Result<Option<String>> {
    if let Some(images) = chart.yaml["annotations"]["artifacthub.io/images"].as_str() {
      let images = YamlLoader::load_from_str(images).with_context(|| {
        format!(
          "Failed to convert `artifacthub.io/images` annotation from {:?}",
          directory
        )
      })?;
      let images = images[0].as_vec().take().with_context(|| {
        format!(
          "Failed to convert `artifacthub.io/images` annotation from {:?}",
          directory
        )
      })?;

      for item in images {
        if let Some(name) = item["name"].as_str() {
          if name == chart.name {
            let image = item["image"]
              .as_str()
              .with_context(|| "image key not found")?;
            let regex = Regex::new(format!(":{}$", chart.app_version).as_str())?;

            if !regex.is_match(image) {
              return Err(anyhow!(
                "The app version does not match the image version in `artifacthub.io/images`"
              ));
            }

            return Ok(Some(image.to_string()));
          }
        }
      }
    }
    Ok(None)
  }
}
