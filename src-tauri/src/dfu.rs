use crate::{
    error::{Error, Result},
    DFUSE_DEFAULT_ADDRESS, USB_BRIDGE_PRODUCT_DFU_ID, USB_BRIDGE_VENDOR_ID,
};
use dfu_libusb::DfuLibusb;
use fs_extra::file::{copy_with_progress, CopyOptions, TransitProcess};
use log::{debug, error};
use std::{path::PathBuf, time::Duration};
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

pub fn install_rpi<F>(binary: PathBuf, progress_handler: F) -> Result<u64>
where
    F: FnMut(TransitProcess),
{
    // sleep to allow disk to mount
    std::thread::sleep(Duration::from_secs(3));

    // get disk info from system
    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks_list());

    // retrieve our disk info
    sys.refresh_disks_list();
    sys.refresh_disks();

    // brittle... but works
    let disks = sys.disks();
    debug!("available disks: {:?}", disks);

    let rpi_disk = disks
        .iter()
        .find(|&disk| disk.is_removable() && disk.name().eq_ignore_ascii_case("RPI-RP2"));

    match rpi_disk {
        Some(disk) => {
            let options = CopyOptions::new().buffer_size(512);
            let destination = disk
                .mount_point()
                .join(PathBuf::from(binary.file_name().unwrap()));

            // Copy binary file path to device
            match copy_with_progress(binary, destination, &options, progress_handler) {
                Ok(bytes_written) => Ok(bytes_written),
                Err(err) => err!(Error::IO(format!("upload failed with reason: {:?}", err))),
            }
        }
        None => err!(Error::Install("UF2 disk not available".to_string())),
    }
}

pub fn install_bridge<F>(binary: PathBuf, progress_handler: F) -> Result<()>
where
    F: FnMut(usize) + 'static,
{
    // open the binary file
    let file = std::fs::File::open(binary)
        .map_err(|e| Error::IO(format!("could not open firmware file: {}", e)))?;

    // create our USB context
    let context = rusb::Context::new()
        .map_err(|e| Error::Install(format!("unable to create usb context: {}", e)))?;

    // open the device
    let (device, handle) = open_device(&context, USB_BRIDGE_VENDOR_ID, USB_BRIDGE_PRODUCT_DFU_ID)?;

    // build the DFU interface
    let mut dfu_iface = DfuLibusb::from_usb_device(device, handle, 0, 0)
        .map_err(|e| Error::Install(e.to_string()))?;

    // setup our progress bar
    dfu_iface
        .with_progress(progress_handler)
        .override_address(DFUSE_DEFAULT_ADDRESS);

    // PERFORM THE INSTALL
    match dfu_iface.download_all(file) {
        Ok(_) => {
            if dfu_iface.will_detach() {
                match dfu_iface.detach() {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        error!("dfu detach error: {}", err);
                        Err(Error::Install(err.to_string()))
                    }
                }
            } else {
                Ok(())
            }
        }
        Err(dfu_libusb::Error::LibUsb(rusb::Error::Io)) => Ok(()),
        Err(err) => {
            error!("dfu download error: {}", err);
            Err(Error::Install(err.to_string()))
        }
    }
}

fn open_device<C: rusb::UsbContext>(
    context: &C,
    vid: u16,
    pid: u16,
) -> Result<(rusb::Device<C>, rusb::DeviceHandle<C>)> {
    let devices = context
        .devices()
        .map_err(|e| Error::USB(format!("unable to enumerate usb devices: {}", e)))?;
    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(x) => x,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            let handle = device
                .open()
                .map_err(|e| Error::USB(format!("unable to open usb device: {}", e)))?;
            return Ok((device, handle));
        }
    }

    Err(Error::USB(format!("unable to find usb device")))
}
