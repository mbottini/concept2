/// Product ID for HID reading.
pub const CONCEPT2_PRODUCT_ID: u16 = 0x04f3;
pub const CONCEPT2_VENDOR_ID: u16 = 0x17a4;

/// Start Flag for Standard Frames.
pub const CSAFE_START_FLAG: u8 = 0xf1;

/// Stop Flag for Frame.
pub const CSAFE_STOP_FLAG: u8 = 0xf2;

/// CSAFE identifiers for both commands and responses.
pub mod CsafeCommands {
    pub const ProprietaryCommand: u8 = 0x1a;
    pub const GetStatus: u8 = 0x80;
    pub const GetVersion: u8 = 0x91;
    pub const GetUserID: u8 = 0x92;
    pub const GetSerialNumber: u8 = 0x94;
    pub const GetOdometer: u8 = 0x9b;
    pub const GetWorkDistance: u8 = 0xa3;
}
