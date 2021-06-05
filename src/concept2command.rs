use crate::consts;

pub enum Concept2Command {
    GetStatus,
    GetVersion,
    GetUserID,
    GetSerialNumber,
    GetOdometer,
    GetHorizontal,
}

impl Concept2Command {
    pub fn iter(&self) -> Box<dyn Iterator<Item = u8>> {
        match self {
            Concept2Command::GetStatus => {
                Box::new(std::iter::once(consts::CsafeCommands::GetStatus))
            }
            Concept2Command::GetVersion => {
                Box::new(std::iter::once(consts::CsafeCommands::GetVersion))
            }
            Concept2Command::GetUserID => {
                Box::new(std::iter::once(consts::CsafeCommands::GetUserID))
            }
            Concept2Command::GetSerialNumber => {
                Box::new(std::iter::once(consts::CsafeCommands::GetSerialNumber))
            }
            Concept2Command::GetOdometer => {
                Box::new(std::iter::once(consts::CsafeCommands::GetOdometer))
            }
            Concept2Command::GetHorizontal => Box::new(
                vec![
                    consts::CsafeCommands::SetUserCfg1,
                    0x1,
                    consts::CsafeCommands::GetHorizontal,
                ]
                .into_iter(),
            ),
        }
    }
}
