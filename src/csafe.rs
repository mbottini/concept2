/// Library for working with CSAFE commands.
///
/// https://en.wikipedia.org/wiki/Communications_Specification_for_Fitness_Equipment
///
/// All Concept2 commands sent through USB are formatted with CSAFE frames.
/// 
/// CSAFE Frames come in two varieties: Standard and Extended. Standard is just data; Extended
/// frames have a destination and source address in addition to the data.  As far as I can tell,
/// we're only concerned with the Standard frames. Extended frames are additional functionality for
/// other equipment that we don't care about.
/// Regardless, everything starts with a Start Flag, then the Data, then the Checksum, and finally
/// the Stop Flag.
/// 
/// The Checksum is gotten by doing bitwise XOR (no carry) with all of the bytes in the command,
/// not including the Start Flag or the Stop Flag.

use crate::concept2command::Concept2Command;
use crate::consts;

pub struct CSAFEFrame {
    command: Concept2Command,
}

fn checksum(command: &Concept2Command) -> u8 {
    command.to_vec()
        .into_iter()
        .fold(0, |x, y| x ^ y)
}

impl CSAFEFrame {
    pub fn new(cmd: Concept2Command) -> CSAFEFrame {
        CSAFEFrame {
            command: cmd
        }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        std::iter::once(consts::CSAFE_START_FLAG)
            .chain(self.command.to_vec().into_iter())
            .chain(std::iter::once(checksum(&self.command)))
            .chain(std::iter::once(consts::CSAFE_STOP_FLAG))
            .collect()
    }
}

mod tests {
    #[test]
    fn test_get_status() {
        let cmd = crate::csafe::CSAFEFrame::new(crate::concept2command::Concept2Command::GetStatus);
        assert_eq!(vec![0xf1, 0x80, 0x80, 0xf2], cmd.to_vec());
    }
}