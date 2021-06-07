/// Product ID for HID reading.
pub const CONCEPT2_PRODUCT_ID: u16 = 0x04f3;
pub const CONCEPT2_VENDOR_ID: u16 = 0x17a4;

/// Start Flag for Standard Frames.
pub const CSAFE_START_FLAG: u8 = 0xf1;

/// Stop Flag for Frame.
pub const CSAFE_STOP_FLAG: u8 = 0xf2;

/// CSAFE identifiers for both commands and responses.
pub mod csafe_commands {
    pub const PROPRIETARY_COMMAND: u8 = 0x1a;
    pub const GET_STATUS: u8 = 0x80;
    pub const GET_VERSION: u8 = 0x91;
    pub const GET_USER_ID: u8 = 0x92;
    pub const GET_SERIAL_NUMBER: u8 = 0x94;
    pub const GET_ODOMETER: u8 = 0x9b;
    pub const GET_WORK_DISTANCE: u8 = 0xa3;
}
