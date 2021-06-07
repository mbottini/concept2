use hidapi::{HidDevice, HidResult};

use crate::consts;
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
    device.write(msg.as_slice()).map(|_| read_hid(&device))
}

pub fn read_hid(device: &HidDevice) -> Vec<u8> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    let mut recv_result: Vec<u8> = vec![0; MSG_LENGTH];
    loop {
        for elem in recv_result.iter_mut() {
            *elem = 0;
        }
        if device.read_timeout(recv_result.as_mut_slice(), 10).is_err()
            || !recv_result.iter().any(|&b| b == consts::CSAFE_START_FLAG)
        {
            break;
        }
        result.push(recv_result.clone());
    }
    result.into_iter().flat_map(|v| v.into_iter()).collect()
}
