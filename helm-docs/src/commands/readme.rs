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
  #[structopt()]
  directory: PathBuf,
}

impl Readme {
  pub fn exec(&self) -> Result<()> {
    if self.write {
      let mut file = File::create(self.directory.join("README.md"))?;
      self.generate_all(&mut file)?;
    } else {
      self.generate_all(&mut std::io::stdout())?;
    }
    Ok(())
  }

  fn generate_all<W: Write>(&self, writer: &mut W) -> Result<()> {
    let chart = Chart::try_from(self.directory.join("Chart.yaml"))?;
    let values = self.directory.join("values.yaml");

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
