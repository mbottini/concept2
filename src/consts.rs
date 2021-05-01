/// Product ID for HID reading.
pub const CONCEPT2_PRODUCT_ID: u16 = 0x04f3;
pub const CONCEPT2_VENDOR_ID: u16 = 0x17a4;

/// CSAFE identifiers for both commands and responses.
pub enum CsafeCommands {
    GetStatus = 0x80,
    GetVersion = 0x91,
    GetSerialNumber = 0x92,
}