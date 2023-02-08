// { id: "16926237606252", vendor_id: 11914, product_id: 61450, description: Some("RP2040"), serial_number: Some("E661343213701439") }
// { id: "17037353476373", vendor_id: 11914, product_id: 3, description: Some("RP2 Boot"), serial_number: Some("E0C912952D54") }
// { id: "16928040556979", vendor_id: 1155, product_id: 22336, description: Some("Bridge 6"), serial_number: Some("208133813536") }

// list of the supported devicees
#[derive(Serialize, Debug, Clone)]
pub enum DeviceType {
    Bridge6,
    Bridge4,
    Click,
    ULoop,
}

#[derive(Serialize, Debug, Clone)]
pub struct Device {
    /// Platform specific unique ID
    pub id: String,
    /// Vendor ID
    pub vendor_id: u16,
    /// Product ID
    pub product_id: u16,
    /// Optional device description
    pub description: Option<String>,
    /// Optional serial number
    pub serial_number: Option<String>,
    /// Supported Device Type
    pub device_type: Option<DeviceType>,
    /// Hardware Version
    pub hardware_version: Option<String>,
}
