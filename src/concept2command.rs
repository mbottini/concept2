use crate::consts;

pub enum Concept2Command {
    GetStatus,
    GetVersion,
    GetUserID,
    GetSerialNumber,
}

impl Concept2Command {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Concept2Command::GetStatus => vec![consts::CsafeCommands::GetStatus as u8],
            Concept2Command::GetVersion => vec![consts::CsafeCommands::GetVersion as u8],
            Concept2Command::GetUserID => vec![consts::CsafeCommands::GetUserID as u8],
            Concept2Command::GetSerialNumber => vec![consts::CsafeCommands::GetSerialNumber as u8],
        }
    }
}
