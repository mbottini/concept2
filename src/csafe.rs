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

use std::boxed::Box;

use crate::concept2command::Concept2Command;

/// Start Flag for Standard Frames.
const START_FLAG: u8 = 0xf1;

/// Stop Flag for Frame. 
const STOP_FLAG: u8 = 0xf2;

pub struct CSAFEFrame {
    command: Box<dyn Concept2Command>,
}

fn checksum(command: &Box<dyn Concept2Command>) -> u8 {
    command.to_vec()
        .into_iter()
        .fold(0, |x, y| x ^ y)
}

impl CSAFEFrame {
    pub fn to_vec(&self) -> Vec<u8> {
        std::iter::once(START_FLAG)
            .chain(self.command.to_vec().into_iter())
            .chain(std::iter::once(checksum(&self.command)))
            .chain(std::iter::once(STOP_FLAG))
            .collect()
    }
}