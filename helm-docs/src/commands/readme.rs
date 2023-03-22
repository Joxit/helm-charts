use crate::table::generate_table;
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
    println!("## Configuration");
    generate_table(values);
  }
}
