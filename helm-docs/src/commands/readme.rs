use crate::table::generate_table;
use crate::usage::generate_usage;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Readme {
  #[structopt()]
  directory: PathBuf,
}

impl Readme {
  pub fn exec(&self) {
    let chart = self.directory.join("Chart.yaml");
    let values = self.directory.join("values.yaml");
    println!("## Usage");
    generate_usage(chart);
    println!("## Configuration");
    generate_table(values);
  }
}
