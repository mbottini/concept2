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
    commands: Vec<Concept2Command>,
}

fn checksum(commands: &Vec<Concept2Command>) -> u8 {
    commands
        .iter()
        .map(|v| v.iter().fold(0, |x, y| x ^ y))
        .fold(0, |x, y| x ^ y)
}

fn stuff_bytes(x: u8) -> Box<dyn Iterator<Item = u8>> {
    match x {
        0xf0 => Box::new(vec![0xf3, 0x00].into_iter()),
        0xf1 => Box::new(vec![0xf3, 0x01].into_iter()),
        0xf2 => Box::new(vec![0xf3, 0x02].into_iter()),
        0xf3 => Box::new(vec![0xf3, 0x03].into_iter()),
        x => Box::new(std::iter::once(x)),
    }
}
impl CSAFEFrame {
    pub fn new(cmds: Vec<Concept2Command>) -> CSAFEFrame {
        CSAFEFrame { commands: cmds }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        std::iter::once(consts::CSAFE_START_FLAG)
            .chain(
                self.commands
                    .iter()
                    .flat_map(|c| c.iter().flat_map(|x| stuff_bytes(x))),
            )
            .chain(std::iter::once(checksum(&self.commands)).flat_map(|x| stuff_bytes(x)))
            .chain(std::iter::once(consts::CSAFE_STOP_FLAG))
            .collect()
    }
}

mod tests {
    #[test]
    fn test_get_status() {
        let cmd =
            crate::csafe::CSAFEFrame::new(vec![crate::concept2command::Concept2Command::GetStatus]);
        assert_eq!(vec![0xf1, 0x80, 0x80, 0xf2], cmd.to_vec());
    }
}
