use crate::error::{Error, Result};
use fs_extra::file::{copy_with_progress, CopyOptions, TransitProcess};
use log::debug;
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

// pub fn install_bridge(handle: AppHandle) {
//     // open the binary file and get the file size
//     let file = std::fs::File::open(binary_path)
//         .map_err(|e| CommandError::IO(format!("could not open firmware file: {}", e)))?;

//     let mut dfu_iface = {
//         //match raw_device {
//         // Some(device) => {
//         //     // get device descriptor
//         //     let (vid, pid) = match device.device_descriptor() {
//         //         Ok(desc) => (desc.vendor_id(), desc.product_id()),
//         //         Err(err) => panic!(
//         //             "unable to get device descriptors from usb device! - error: {}",
//         //             err
//         //         ),
//         //     };
//         //     // open the DFU interface
//         //     info!("opening interface: {:#06x}:{:#06x}", vid, pid);
//         //     DfuLibusb::open(device.context(), vid, pid, 0, 0)
//         //         .map_err(|e| CommandError::Dfu(e.to_string()))?
//         // }
//         // // if we didn't pass in a device, just try to guess via VID and PID
//         // None => {
//         // create new usb context
//         info!("device was not passed in - creating new usb context");
//         let context = rusb::Context::new()
//             .map_err(|e| CommandError::Device(format!("unable to create usb context: {}", e)))?;
//         // open the DFU interface
//         DfuLibusb::open(&context, USB_VENDOR_ID, USB_PRODUCT_DFU_ID, 0, 0)
//             .map_err(|e| CommandError::Dfu(e.to_string()))?
//         //}
//     };

//     // setup our progress bar - if available
//     // if progress.is_some() {
//     //     dfu_iface.with_progress(progress.unwrap());
//     // }

//     // PERFORM THE INSTALL
//     match dfu_iface.download_all(file) {
//         Ok(_) => Ok(()),
//         Err(dfu_libusb::Error::LibUsb(rusb::Error::Io)) => Ok(()),
//         Err(err) => {
//             error!("dfu download error: {}", err);
//             Err(CommandError::Dfu(err.to_string()))
//         }
//     }
// }
