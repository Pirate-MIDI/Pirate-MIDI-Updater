use crate::{
    error::{Error, Result},
    USB_BRIDGE_PRODUCT_DFU_ID, USB_BRIDGE_VENDOR_ID,
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

    // create + open the DFU interface
    let mut dfu_iface = {
        let context = rusb::Context::new()
            .map_err(|e| Error::Install(format!("unable to create usb context: {}", e)))?;
        // open the DFU interface
        DfuLibusb::open(
            &context,
            USB_BRIDGE_VENDOR_ID,
            USB_BRIDGE_PRODUCT_DFU_ID,
            0,
            0,
        )
        .map_err(|e| Error::Install(e.to_string()))?
    };

    // setup our progress bar
    dfu_iface.with_progress(progress_handler);

    // PERFORM THE INSTALL
    match dfu_iface.download_all(file) {
        Ok(_) => Ok(()),
        Err(dfu_libusb::Error::LibUsb(rusb::Error::Io)) => Ok(()),
        Err(err) => {
            error!("dfu download error: {}", err);
            Err(Error::Install(err.to_string()))
        }
    }
}
