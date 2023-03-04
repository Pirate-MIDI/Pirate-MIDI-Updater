use regex::Regex;
use std::{
    ffi::{CStr, CString},
    mem::size_of,
    ptr::null,
    ptr::null_mut,
};
use windows_sys::{
    core::GUID,
    Win32::{
        Devices::{
            DeviceAndDriverInstallation::{
                SetupDiClassGuidsFromNameA, SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo,
                SetupDiGetClassDevsW, SetupDiGetDeviceInstanceIdA, SetupDiGetDevicePropertyW,
                SetupDiGetDeviceRegistryPropertyW, SetupDiOpenDevRegKey, DICS_FLAG_GLOBAL,
                DIGCF_PRESENT, DIREG_DEV, SPDRP_FRIENDLYNAME, SPDRP_HARDWAREID, SPDRP_MFG,
                SP_DEVINFO_DATA,
            },
            Properties::{DEVPKEY_Device_Parent, DEVPROPKEY},
        },
        Foundation::{GetLastError, MAX_PATH},
        System::{
            Diagnostics::Debug::{
                FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
            },
            Registry::{RegCloseKey, RegQueryValueExW, KEY_READ},
        },
    },
};

/// Contains all possible USB information about a `SerialPort`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsbPortInfo {
    /// Vendor ID
    pub vid: u16,
    /// Product ID
    pub pid: u16,
    /// Serial number (arbitrary string)
    pub serial_number: Option<String>,
    /// Manufacturer (arbitrary string)
    pub manufacturer: Option<String>,
    /// Product name (arbitrary string)
    pub product: Option<String>,
    /// Interface (id number for multiplexed devices)
    pub interface: Option<u8>,
}

/// The physical type of a `SerialPort`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SerialPortType {
    /// The serial port is connected via USB
    UsbPort(UsbPortInfo),
    /// The serial port is connected via PCI (permanent port)
    PciPort,
    /// The serial port is connected via Bluetooth
    BluetoothPort,
    /// It can't be determined how the serial port is connected
    Unknown,
}

/// A device-independent implementation of serial port information
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SerialPortInfo {
    /// The short name of the serial port
    pub port_name: String,
    /// The hardware device type that exposes this port
    pub port_type: SerialPortType,
}

// gets the guids for the COM ports class
fn get_port_class_guids() -> Result<Vec<GUID>, &'static str> {
    let class_name = CString::new("Ports").unwrap();
    let mut buffer: Vec<GUID> = vec![GUID::from_u128(0); 1];
    let mut expected_length = buffer.len() as u32;

    // get all guids assoicated with COM ports, and insert them into the guids vec
    // we initially assume there is at least one.
    if unsafe {
        SetupDiClassGuidsFromNameA(
            class_name.as_ptr() as *const u8,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            &mut expected_length,
        )
    } < 1
    {
        // throw error, but do nothing right now
        println!("unable to get GUIDs");
    }

    // if we have more guids, expand our vec to handle all the guids available
    if expected_length > buffer.capacity() as u32 {
        buffer.resize(expected_length as usize, GUID::from_u128(0));

        // grab any additional guids
        if unsafe {
            SetupDiClassGuidsFromNameA(
                class_name.as_ptr() as *const u8,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                &mut expected_length,
            )
        } < 1
        {
            // throw error, but do nothing for now
            println!("unable to get GUIDs a second time");
        }
    }

    Ok(buffer)
}

fn get_last_error() -> String {
    let err = unsafe { GetLastError() };
    if err == 0 {
        return String::new();
    }

    let mut buffer: Vec<u16> = vec![0; MAX_PATH as usize];

    unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            null(),
            err,
            0,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            null_mut(),
        )
    };

    String::from_utf16_lossy(&buffer)
        .trim_end_matches(0 as char)
        .to_string()
}

/// Windows usb port information can be determined by the port's HWID string.
///
/// This function parses the HWID string using regex, and returns the USB port
/// information if the hardware ID can be parsed correctly. The manufacturer
/// and product names cannot be determined from the HWID string, so those are
/// set as None.
///
/// Some HWID examples are:
///   - MicroPython pyboard:    USB\VID_F055&PID_9802\385435603432
///   - BlackMagic GDB Server:  USB\VID_1D50&PID_6018&MI_00\6&A694CA9&0&0000
///   - BlackMagic UART port:   USB\VID_1D50&PID_6018&MI_02\6&A694CA9&0&0002
///   - FTDI Serial Adapter:    FTDIBUS\VID_0403+PID_6001+A702TB52A\0000
fn parse_usb_port_info(hardware_id: &str) -> Option<UsbPortInfo> {
    println!("parsing: {:?}", hardware_id);
    let re = Regex::new(concat!(
        r"VID_(?P<vid>[[:xdigit:]]{4})",
        r"[&+]PID_(?P<pid>[[:xdigit:]]{4})",
        r"(?:[&+]MI_(?P<iid>[[:xdigit:]]{2})){0,1}",
        r"([\\+](?P<serial>\w+))?"
    ))
    .unwrap();

    let caps = re.captures(hardware_id)?;

    Some(UsbPortInfo {
        vid: u16::from_str_radix(&caps[1], 16).ok()?,
        pid: u16::from_str_radix(&caps[2], 16).ok()?,
        serial_number: caps.name("serial").map(|m| m.as_str().to_string()),
        manufacturer: None,
        product: None,
        interface: caps
            .name("iid")
            .and_then(|m| u8::from_str_radix(m.as_str(), 16).ok()),
    })
}

struct PortDevices {
    /// Handle to a device information set.
    hdi: isize,

    /// Index used by iterator.
    dev_idx: u32,
}

impl PortDevices {
    // Creates PortDevices object which represents the set of devices associated with a particular
    // Ports class (given by `guid`).
    pub fn new(guid: &GUID) -> Self {
        PortDevices {
            hdi: unsafe { SetupDiGetClassDevsW(guid, null(), 0, DIGCF_PRESENT) },
            dev_idx: 0,
        }
    }
}

impl Iterator for PortDevices {
    type Item = PortDevice;

    /// Iterator which returns a PortDevice from the set of PortDevices associated with a
    /// particular PortDevices class (guid).
    fn next(&mut self) -> Option<PortDevice> {
        let mut port_dev = PortDevice {
            hdi: self.hdi,
            devinfo_data: SP_DEVINFO_DATA {
                cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
                ClassGuid: GUID::from_u128(0),
                DevInst: 0,
                Reserved: 0,
            },
        };
        if unsafe { SetupDiEnumDeviceInfo(self.hdi, self.dev_idx, &mut port_dev.devinfo_data) } < 1
        {
            None
        } else {
            self.dev_idx += 1;
            Some(port_dev)
        }
    }
}

impl Drop for PortDevices {
    fn drop(&mut self) {
        // Release the PortDevices object allocated in the constructor.
        unsafe {
            SetupDiDestroyDeviceInfoList(self.hdi);
        }
    }
}

struct PortDevice {
    /// Handle to a device information set.
    hdi: isize,

    /// Information associated with this device.
    pub devinfo_data: SP_DEVINFO_DATA,
}

impl PortDevice {
    // Retrieves the device instance id string associated with this device. Some examples of
    // instance id strings are:
    //  MicroPython Board:  USB\VID_F055&PID_9802\385435603432
    //  FTDI USB Adapter:   FTDIBUS\VID_0403+PID_6001+A702TB52A\0000
    //  Black Magic Probe (Composite device with 2 UARTS):
    //      GDB Port:       USB\VID_1D50&PID_6018&MI_00\6&A694CA9&0&0000
    //      UART Port:      USB\VID_1D50&PID_6018&MI_02\6&A694CA9&0&0002
    fn instance_id(&mut self) -> Option<String> {
        let mut buffer: Vec<u8> = vec![0u8; MAX_PATH as usize];
        if unsafe {
            SetupDiGetDeviceInstanceIdA(
                self.hdi,
                &mut self.devinfo_data,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                null_mut(),
            )
        } < 1
        {
            // Try to retrieve hardware id property.
            self.get_device_registry_property(SPDRP_HARDWAREID)
        } else {
            Some(unsafe {
                CStr::from_ptr(buffer.as_ptr() as *mut i8)
                    .to_string_lossy()
                    .into_owned()
            })
        }
    }

    // Determines the port_type for this device, and if it's a USB port populate the various fields.
    pub fn port_type(&mut self) -> SerialPortType {
        self.get_device_property(&DEVPKEY_Device_Parent)
            .and_then(|s| parse_usb_port_info(&s))
            .map(|mut info| {
                info.manufacturer = self.get_device_registry_property(SPDRP_MFG);
                info.product = self.get_device_registry_property(SPDRP_FRIENDLYNAME);
                SerialPortType::UsbPort(info)
            })
            .unwrap_or(SerialPortType::Unknown)
    }
    // Retrieves the port name (i.e. COM6) associated with this device.
    pub fn name(&mut self) -> String {
        self.get_registry_value("PortName")
    }

    fn get_device_registry_property(&mut self, property: u32) -> Option<String> {
        let mut buffer: Vec<u16> = vec![0; MAX_PATH as usize];

        if unsafe {
            SetupDiGetDeviceRegistryPropertyW(
                self.hdi,
                &mut self.devinfo_data,
                property,
                null_mut(),
                buffer.as_mut_ptr() as *mut u8,
                buffer.len() as u32,
                null_mut(),
            )
        } < 1
        {
            println!("{}", get_last_error());
            return None;
        }

        // convert our buffer to a string
        String::from_utf16_lossy(&buffer)
            .trim_end_matches(0 as char)
            .split(';')
            .last()
            .map(str::to_string)
    }

    fn get_device_property(&mut self, property: &DEVPROPKEY) -> Option<String> {
        let mut buffer: Vec<u16> = vec![0u16; MAX_PATH as usize];
        let mut output_type: u32 = 0u32;
        let mut required_size: u32 = 0u32;
        if unsafe {
            SetupDiGetDevicePropertyW(
                self.hdi,
                &mut self.devinfo_data,
                property,
                &mut output_type,
                buffer.as_mut_ptr() as *mut u8,
                buffer.len() as u32,
                &mut required_size,
                0,
            )
        } < 1
        {
            println!("{}", get_last_error());
            return None;
        };

        // convert our buffer to a string
        Some(
            String::from_utf16_lossy(&buffer[0..required_size as usize])
                .trim_end_matches(0 as char)
                .to_string(),
        )
    }

    fn get_registry_value(&mut self, key: &str) -> String {
        let value_name: Vec<u16> = key.encode_utf16().chain(Some(0)).collect();
        let mut buffer: Vec<u16> = vec![0u16; MAX_PATH as usize];
        let mut expected_size: u32 = buffer.len() as u32;

        // open the registry
        let hkey = unsafe {
            SetupDiOpenDevRegKey(
                self.hdi,
                &mut self.devinfo_data,
                DICS_FLAG_GLOBAL,
                0,
                DIREG_DEV,
                KEY_READ,
            )
        };
        // query the registry
        unsafe {
            RegQueryValueExW(
                hkey,
                value_name.as_ptr(),
                null_mut(),
                null_mut(),
                buffer.as_mut_ptr() as *mut u8,
                &mut expected_size,
            )
        };
        // close the registry
        unsafe { RegCloseKey(hkey) };

        // convert our buffer to a string
        let result = &buffer[0..expected_size as usize];

        String::from_utf16_lossy(result)
            .trim_end_matches(0 as char)
            .to_string()
    }
}

pub fn available_ports() -> Vec<SerialPortInfo> {
    let mut ports: Vec<SerialPortInfo> = Vec::new();
    match get_port_class_guids() {
        Ok(guids) => {
            for guid in guids {
                let port_devices = PortDevices::new(&guid);
                for mut port_device in port_devices {
                    let port_name = port_device.name();

                    debug_assert!(
                        port_name.as_bytes().last().map_or(true, |c| *c != b'\0'),
                        "port_name has a trailing nul: {:?}",
                        port_name
                    );

                    // This technique also returns parallel ports, so we filter these out.
                    if port_name.starts_with("LPT") {
                        continue;
                    }

                    ports.push(SerialPortInfo {
                        port_name: port_name,
                        port_type: port_device.port_type(),
                    });
                }
            }
        }
        Err(err) => println!("some error: {:?}", err),
    }

    ports
}
