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
    GetWorkoutType,
    GetWorkDistance,
}

impl From<Concept2ProprietaryCommand> for u8 {
    fn from(c: Concept2ProprietaryCommand) -> Self {
        match c {
            Concept2ProprietaryCommand::GetWorkDistance => {
                consts::csafe_commands::GET_WORK_DISTANCE
            }
            Concept2ProprietaryCommand::GetWorkoutType => consts::csafe_commands::GET_WORKOUT_TYPE,
        }
    }
}

impl<'a> From<&'a Concept2ProprietaryCommand> for u8 {
    fn from(c: &'a Concept2ProprietaryCommand) -> Self {
        match c {
            Concept2ProprietaryCommand::GetWorkDistance => {
                consts::csafe_commands::GET_WORK_DISTANCE
            }
            Concept2ProprietaryCommand::GetWorkoutType => consts::csafe_commands::GET_WORKOUT_TYPE,
        }
    }
}

impl Concept2Command {
    pub fn iter(&self) -> Box<dyn Iterator<Item = u8> + '_> {
        match self {
            Concept2Command::GetStatus => {
                Box::new(std::iter::once(consts::csafe_commands::GET_STATUS))
            }
            Concept2Command::GetVersion => {
                Box::new(std::iter::once(consts::csafe_commands::GET_VERSION))
            }
            Concept2Command::GetUserID => {
                Box::new(std::iter::once(consts::csafe_commands::GET_USER_ID))
            }
            Concept2Command::GetSerialNumber => {
                Box::new(std::iter::once(consts::csafe_commands::GET_SERIAL_NUMBER))
            }
            Concept2Command::GetOdometer => {
                Box::new(std::iter::once(consts::csafe_commands::GET_ODOMETER))
            }
            Concept2Command::ProprietaryCommand(vec) => Box::new(
                std::iter::once(consts::csafe_commands::PROPRIETARY_COMMAND)
                    .chain(std::iter::once(vec.len() as u8))
                    .chain(vec.iter().map(|c| u8::from(c))),
            ),
        }
    }
}
