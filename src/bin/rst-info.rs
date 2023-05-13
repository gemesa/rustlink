use anyhow::{bail, Error, Result};
use clap::{Parser, Subcommand};

use std::time::Duration;

use usb_ids::FromId;

use rusb::{DeviceDescriptor, DeviceHandle, DeviceList, Language, UsbContext};

struct UsbDevice<T: UsbContext> {
    handle: DeviceHandle<T>,
    language: Language,
    timeout: Duration,
}

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
    let timeout = Duration::from_secs(1);

    for device in DeviceList::new()?.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        let mut usb_device = {
            match device.open() {
                Ok(h) => match h.read_languages(timeout) {
                    Ok(l) => {
                        if !l.is_empty() {
                            Some(UsbDevice {
                                handle: h,
                                language: l[0],
                                timeout,
                            })
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                },
                Err(_) => None,
            }
        };

        if device_desc.vendor_id() == VENDOR_ID_ST {
            print_device(&device_desc, &mut usb_device);
        }
    }

    Ok(())
}

fn print_device<T: UsbContext>(device_desc: &DeviceDescriptor, handle: &mut Option<UsbDevice<T>>) {
    let vid: u16 = device_desc.vendor_id();
    let pid = device_desc.product_id();

    let vendor_name = match usb_ids::Vendor::from_id(device_desc.vendor_id()) {
        Some(vendor) => vendor.name(),
        None => "Unknown vendor",
    };

    let product_name =
        match usb_ids::Device::from_vid_pid(device_desc.vendor_id(), device_desc.product_id()) {
            Some(product) => product.name(),
            None => "Unknown product",
        };

    println!("Device Descriptor:");
    println!(
        "  bcdUSB             {:2}.{}{}",
        device_desc.usb_version().major(),
        device_desc.usb_version().minor(),
        device_desc.usb_version().sub_minor()
    );
    println!("  idVendor          {vid:#06x} {vendor_name}",);
    println!("  idProduct         {pid:#06x} {product_name}",);
    println!(
        "  bcdDevice          {:2}.{}{}",
        device_desc.device_version().major(),
        device_desc.device_version().minor(),
        device_desc.device_version().sub_minor()
    );
    println!(
        "  iManufacturer        {:3} {}",
        device_desc.manufacturer_string_index().unwrap_or(0),
        handle.as_mut().map_or(String::new(), |h| h
            .handle
            .read_manufacturer_string(h.language, device_desc, h.timeout)
            .unwrap_or_default())
    );
    println!(
        "  iProduct             {:3} {}",
        device_desc.product_string_index().unwrap_or(0),
        handle.as_mut().map_or(String::new(), |h| h
            .handle
            .read_product_string(h.language, device_desc, h.timeout)
            .unwrap_or_default())
    );
    println!(
        "  iSerialNumber        {:3} {}",
        device_desc.serial_number_string_index().unwrap_or(0),
        handle.as_mut().map_or(String::new(), |h| h
            .handle
            .read_serial_number_string(h.language, device_desc, h.timeout)
            .unwrap_or_default())
    );
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => list_st_devices(),
        None => bail!("unrecognized command"),
    }
}
