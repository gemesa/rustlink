use anyhow::{bail, Context, Error, Result};
use clap::{Parser, Subcommand};
use probe_rs::{flashing::FileDownloadError, Permissions, Probe};
use probe_rs_cli_util::{
    common_options::{CargoOptions, FlashOptions, ProbeOptions},
    flash::run_flash_download,
};
use std::fs::File;
use std::path::Path;

/// Programmer and flash manipulation tool

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Download memory to attached target
    Download {
        #[clap(flatten)]
        common: ProbeOptions,

        /// Serial number of STLink
        serial: String,

        /// Target chip
        target: String,

        /// Format of the file to be downloaded to the flash. Possible values are case-insensitive.
        #[clap(value_enum, ignore_case = true, default_value = "elf", long)]
        format: DownloadFileType,

        /// The path to the file to be downloaded to the flash
        path: String,

        /// Whether to erase the entire chip before downloading
        #[clap(long)]
        chip_erase: bool,

        /// Whether to disable fancy progress reporting
        #[clap(long)]
        disable_progressbars: bool,

        /// Disable double-buffering when downloading flash.  If downloading times out, try this option.
        #[clap(long = "disable-double-buffering")]
        disable_double_buffering: bool,
    },
}

#[allow(clippy::too_many_arguments)]
fn download_program_fast(
    common: ProbeOptions,
    serial: &str,
    target: &str,
    format: DownloadFileType,
    path: &str,
    do_chip_erase: bool,
    disable_progressbars: bool,
    disable_double_buffering: bool,
) -> Result<()> {
    let probes = Probe::list_all();

    let probe = probes
        .into_iter()
        .find(|probe| probe.serial_number == Some(serial.to_owned()));

    if probe.is_none() {
        bail!("no STLink device found with serial number {}", serial)
    }

    let probe = probe.unwrap().open()?;

    let mut session = probe.attach(target, Permissions::default())?;

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(FileDownloadError::IO(e)).context("failed to open binary file"),
    };

    let mut loader = session.target().flash_loader();

    match format {
        DownloadFileType::Elf => loader.load_elf_data(&mut file),
        DownloadFileType::Hex => loader.load_hex_data(&mut file),
    }?;

    run_flash_download(
        &mut session,
        Path::new(path),
        &FlashOptions {
            list_chips: false,
            list_probes: false,
            disable_progressbars,
            disable_double_buffering,
            reset_halt: false,
            log: None,
            restore_unwritten: false,
            flash_layout_output_path: None,
            elf: None,
            work_dir: None,
            cargo_options: CargoOptions::default(),
            probe_options: common,
        },
        loader,
        do_chip_erase,
    )?;

    Ok(())
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
enum DownloadFileType {
    Elf,
    Hex,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Download {
            common,
            serial,
            target,
            format,
            path,
            chip_erase,
            disable_progressbars,
            disable_double_buffering,
        }) => download_program_fast(
            common,
            &serial,
            &target,
            format,
            &path,
            chip_erase,
            disable_progressbars,
            disable_double_buffering,
        ),
        None => bail!("unrecognized command"),
    }
}
