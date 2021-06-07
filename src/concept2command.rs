use crate::consts;

pub enum Concept2Command {
    GetStatus,
    GetVersion,
    GetUserID,
    GetSerialNumber,
    GetOdometer,
    ProprietaryCommand(Vec<Concept2ProprietaryCommand>),
}

#[derive(Clone, PartialEq, Eq)]
pub enum Concept2ProprietaryCommand {
    GetWorkDistance,
}

impl From<Concept2ProprietaryCommand> for u8 {
    fn from(c: Concept2ProprietaryCommand) -> Self {
        match c {
            Concept2ProprietaryCommand::GetWorkDistance => consts::CsafeCommands::GetWorkDistance,
        }
    }
}

impl<'a> From<&'a Concept2ProprietaryCommand> for u8 {
    fn from(c: &'a Concept2ProprietaryCommand) -> Self {
        match c {
            Concept2ProprietaryCommand::GetWorkDistance => consts::CsafeCommands::GetWorkDistance,
        }
    }
}

impl Concept2Command {
    pub fn iter(&self) -> Box<dyn Iterator<Item = u8> + '_> {
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
            Concept2Command::ProprietaryCommand(vec) => Box::new(
                std::iter::once(consts::CsafeCommands::ProprietaryCommand)
                    .chain(std::iter::once(vec.len() as u8))
                    .chain(vec.iter().map(|c| u8::from(c))),
            ),
        }
    }
}
