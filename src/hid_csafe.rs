use hidapi::{HidDevice, HidResult};

use crate::concept2response;
use crate::csafe;

const MSG_LENGTH: usize = 121;

pub fn write_read_csafe_cmd(
    device: &HidDevice,
    report_num: u8,
    cmd: &csafe::CSAFEFrame,
) -> HidResult<Vec<u8>> {
    let msg: Vec<u8> = std::iter::once(report_num)
        .chain(cmd.to_vec().into_iter())
        .chain(std::iter::repeat(0))
        .take(MSG_LENGTH)
        .collect();
    let mut recv_result: Vec<u8> = vec![0; MSG_LENGTH];
    device
        .write(msg.as_slice())
        .map(|_| device.read(recv_result.as_mut_slice()))
        .map(|_| recv_result)
}
