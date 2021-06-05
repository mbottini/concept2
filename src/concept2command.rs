use crate::consts;

pub enum Concept2Command {
    GetStatus,
    GetVersion,
    GetUserID,
    GetSerialNumber,
}

impl Concept2Command {
    pub fn iter(&self) -> impl Iterator<Item = u8> {
        match self {
            Concept2Command::GetStatus => std::iter::once(consts::CsafeCommands::GetStatus),
            Concept2Command::GetVersion => std::iter::once(consts::CsafeCommands::GetVersion),
            Concept2Command::GetUserID => std::iter::once(consts::CsafeCommands::GetUserID),
            Concept2Command::GetSerialNumber => {
                std::iter::once(consts::CsafeCommands::GetSerialNumber)
            }
        }
    }
}
