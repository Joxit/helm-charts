use crate::chart::Chart;
use crate::prerequisites::generate_prerequisites;
use crate::table::generate_table;
use crate::usage::generate_usage;
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Readme {
  #[structopt()]
  directory: PathBuf,
}

impl Readme {
  pub fn exec(&self) -> Result<()> {
    let chart = Chart::try_from(self.directory.join("Chart.yaml"))?;
    let values = self.directory.join("values.yaml");

    println!("# {} Chart", chart.pretty_name);
    println!("");
    println!("## Prerequisites");
    generate_prerequisites();
    println!("## Usage");
    generate_usage(chart);
    println!("## Configuration");
    generate_table(values)?;

    Ok(())
  }
}
