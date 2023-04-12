use crate::chart::Chart;
use crate::prerequisites::generate_prerequisites;
use crate::table::generate_table;
use crate::usage::generate_usage;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Readme {
  #[structopt(short = "w", long = "write")]
  write: bool,
  #[structopt(required = true)]
  directories: Vec<PathBuf>,
}

impl Readme {
  pub fn exec(&self) -> Result<()> {
    for directory in &self.directories {
      if self.write {
        let mut file = File::create(directory.join("README.md"))?;
        self.generate_readme(&mut file, &directory)?;
      } else {
        self.generate_readme(&mut std::io::stdout(), &directory)?;
      }
    }
    Ok(())
  }

  fn generate_readme<W: Write>(&self, writer: &mut W, directory: &PathBuf) -> Result<()> {
    let chart = Chart::try_from(directory.join("Chart.yaml"))?;
    let values = directory.join("values.yaml");

    writeln!(writer, "# {} Chart", chart.pretty_name)?;
    writeln!(writer, "")?;
    writeln!(writer, "## Prerequisites")?;
    generate_prerequisites(writer)?;
    writeln!(writer, "## Usage")?;
    generate_usage(writer, chart)?;
    writeln!(writer, "## Configuration")?;
    generate_table(writer, values)?;
    Ok(())
  }
}
