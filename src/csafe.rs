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

/// Checksum computes a single byte with exclusive OR on
/// all Concept2Commands in the slice.
fn checksum(commands: &[Concept2Command]) -> u8 {
    commands
        .iter()
        .map(|v| v.iter().fold(0, |x, y| x ^ y))
        .fold(0, |x, y| x ^ y)
}

/// stuff_bytes is CSAFE's response to the fact that its start and end flags
/// could very well be actual data - it's very possible that `0xf0` could be
/// some meter total or whatever. As a result, whenever one of these bytes
/// is in data, the byte is replaced with *two* bytes - `0xf3` and another
/// byte. This is also done in reverse from the Concept2 machine; the response
/// will have to be *unstuffed*.
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
    /// Resolves a CSAFEFrame to a vector of bytes. Every frame must start with a start flag,
    /// contain the byte representation of each CSAFE command, a checksum, and the stop flag.
    pub fn to_vec(&self) -> Vec<u8> {
        std::iter::once(consts::CSAFE_START_FLAG)
            .chain(
                self.commands
                    .iter()
                    .flat_map(|c| c.iter().flat_map(stuff_bytes)),
            )
            .chain(std::iter::once(checksum(&self.commands)).flat_map(stuff_bytes))
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
