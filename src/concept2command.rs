use crate::consts;

pub trait Concept2Command {
    fn to_vec(&self) -> Vec<u8>;
}

/// Wrapper of CSAFE_GETSTATUS_CMD, command 0x80.
/// 
/// Return struct will consist of a single byte denoting the CSAFE status.
pub struct GetStatus;

impl Concept2Command for GetStatus {
    fn to_vec(&self) -> Vec<u8> {
        vec![consts::CsafeCommands::GetStatus as u8]
    }
}

pub struct GetVersion;

impl Concept2Command for GetVersion {
    fn to_vec(&self) -> Vec<u8> {
        vec![consts::CsafeCommands::GetVersion as u8]
    }
}

pub struct GetSerialNumber;

impl Concept2Command for GetSerialNumber {
    fn to_vec(&self) -> Vec<u8> {
        vec![consts::CsafeCommands::GetSerialNumber as u8]
    }
}
