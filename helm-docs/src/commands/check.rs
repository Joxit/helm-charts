use crate::chart::Chart;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use yaml_rust::{Yaml, YamlLoader};

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
    let values = self.get_values_yaml(&directory)?;

    if let Some(image_tag) = self.check_chart_file_image(&chart, directory)? {
      if let Some(image) = image_tag.split(":").next() {
        self.check_values_file_image(image, &image_tag, &values)?;
      }
    }

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

  fn get_values_yaml(&self, directory: &PathBuf) -> Result<Yaml> {
    let file = directory.join("values.yaml");
    let content =
      fs::read_to_string(&file).with_context(|| format!("Failed to open {:?}", file))?;

    let values = YamlLoader::load_from_str(&content)
      .with_context(|| format!("Failed to parse YAML file {:?}", file))?;
    Ok(values[0].clone())
  }

  fn check_values_file_image(&self, image: &str, image_tag: &str, values: &Yaml) -> Result<()> {
    if let Some(map) = values.as_hash() {
      for key in map.keys() {
        let value = map[key].as_str().unwrap_or("");
        if key.as_str() == Some("image") && value.starts_with(image) {
          if value != image_tag {
            return Err(anyhow!(
              "Incorrect version in your values.yaml, should be {} and found {}",
              image_tag,
              value
            ));
          }
        } else {
          self.check_values_file_image(image, image_tag, &map[key])?;
        }
      }
    }
    Ok(())
  }
}
