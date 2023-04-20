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

pub fn generate_readme<W: Write>(writer: &mut W, directory: &PathBuf) -> Result<()> {
  let chart = Chart::try_from(directory.join("Chart.yaml"))?;
  let values = directory.join("values.yaml");

  writeln!(writer, "# {} Chart", chart.pretty_name)?;
  writeln!(writer, "")?;
  writeln!(writer, "## Prerequisites")?;
  writeln!(writer, "{}", generate_prerequisites()?)?;
  writeln!(writer, "## Usage")?;
  writeln!(writer, "{}", generate_usage(chart)?)?;
  writeln!(writer, "## Configuration")?;
  write!(writer, "{}", generate_table(values)?)?;
  Ok(())
}
