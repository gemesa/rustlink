use anyhow::{bail, Error, Result};
use clap::{Parser, Subcommand};
use probe_rs::Probe;

const VENDOR_ID_ST: u16 = 0x0483;

/// Chip and device information tool

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List ST devices
    List,
}

fn list_st_devices() -> Result<()> {
    let mut probes = Probe::list_all();

    if probes.is_empty() {
        bail!("No device found.")
    }

    probes.retain(|probe| probe.vendor_id == VENDOR_ID_ST);

    if probes.is_empty() {
        bail!("No ST device found.")
    }

    println!("The following devices were found:");
    for probe in probes {
        println!("{:?}", probe);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => list_st_devices(),
        None => bail!("unrecognized command"),
    }
}
