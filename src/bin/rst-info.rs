use anyhow::{bail, Error, Result};
use clap::{Parser, Subcommand};
use probe_rs::Probe;

const VENDOR_ID_ST: u16 = 0x0483;
const PRODUCT_ID_STLINK: u16 = 0x3748;

/// Chip and device information tool

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List STLink devices
    List,
}

fn list_stlink_devices() -> Result<()> {
    let mut probes = Probe::list_all();

    if probes.is_empty() {
        bail!("no device found")
    }

    probes.retain(|probe| {
        (probe.vendor_id == VENDOR_ID_ST) && (probe.product_id == PRODUCT_ID_STLINK)
    });

    if probes.is_empty() {
        bail!("no STLink device found")
    }

    probes.iter().enumerate().for_each(|(num, probe)| {
        println!(
            "[{}]: {} - serial: {}",
            num,
            probe.identifier,
            probe.serial_number.clone().unwrap()
        )
    });

    Ok(())
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => list_stlink_devices(),
        None => bail!("unrecognized command"),
    }
}
