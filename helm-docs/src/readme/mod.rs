use self::prerequisites::generate_prerequisites;
use self::table::generate_table;
use self::usage::generate_usage;
use crate::chart::Chart;
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;

mod prerequisites;
mod table;
mod usage;

pub struct Readme {
  chart: Chart,
  values: PathBuf,
}

impl Readme {
  pub fn new(directory: &PathBuf) -> Result<Self> {
    Ok(Self {
      chart: Chart::try_from(directory.join("Chart.yaml"))?,
      values: directory.join("values.yaml"),
    })
  }

  pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
    writeln!(writer, "# {} Chart", &self.chart.pretty_name)?;
    writeln!(writer, "")?;
    writeln!(writer, "## Prerequisites")?;
    writeln!(writer, "{}", generate_prerequisites()?)?;
    writeln!(writer, "## Usage")?;
    writeln!(writer, "{}", generate_usage(&self.chart)?)?;
    writeln!(writer, "## Configuration")?;
    write!(writer, "{}", generate_table(&self.values)?)?;
    Ok(())
  }
}
