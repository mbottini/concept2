use crate::consts;
use std::convert::TryInto;

#[derive(Debug, PartialEq, Eq)]
pub enum Concept2Response {
    GetStatus,
    GetVersion,
    GetUserID(String),
    GetSerialNumber(String),
    GetOdometer(u32, u8),
    ProprietaryCommand(Vec<Concept2ResponseProprietary>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Concept2ResponseProprietary {
    GetWorkDistance(u32, u8),
    GetWorkoutType(u8),
}

pub struct ResponseFrame {
    _status: u8,
    identifier: u8,
    bytes: u8,
    data: Vec<u8>,
}

impl ResponseFrame {
    pub fn parse(self) -> Option<Concept2Response> {
        match self.identifier {
            consts::csafe_commands::GET_USER_ID => {
                assert!(self.bytes == 5);
                Some(Concept2Response::GetUserID(
                    String::from_utf8(self.data).expect("parse error"),
                ))
            }
            consts::csafe_commands::GET_SERIAL_NUMBER => {
                assert!(self.bytes == 9);
                Some(Concept2Response::GetSerialNumber(
                    String::from_utf8(self.data).expect("parse error"),
                ))
            }
            consts::csafe_commands::GET_ODOMETER => {
                assert!(self.bytes == 5);
                let distance: u32 = u32::from_le_bytes(
                    self.data
                        .iter()
                        .cloned()
                        .take(4)
                        .collect::<Vec<u8>>()
                        .as_slice()
                        .try_into()
                        .expect("incorrect slice length"),
                );
                let units: u8 = *self.data.last().unwrap();
                Some(Concept2Response::GetOdometer(distance, units))
            }
            consts::csafe_commands::PROPRIETARY_COMMAND => parse_proprietary(self.data),
            _ => None,
        }
    }
}

fn parse_proprietary(vec: Vec<u8>) -> Option<Concept2Response> {
    let mut proprietary_vec: Vec<Concept2ResponseProprietary> = Vec::new();
    let mut vec_iter = vec.into_iter();
    while let Some(identifier) = vec_iter.next() {
        match identifier {
            consts::csafe_commands::GET_WORK_DISTANCE => {
                if vec_iter.next() != Some(5) {
                    return None;
                }
                proprietary_vec.push(Concept2ResponseProprietary::GetWorkDistance(
                    u32::from_le_bytes(
                        vec_iter
                            .by_ref()
                            .take(4)
                            .collect::<Vec<u8>>()
                            .as_slice()
                            .try_into()
                            .expect("incorrect slice length"),
                    ),
                    vec_iter.next().unwrap(),
                ));
            }
            consts::csafe_commands::GET_WORKOUT_TYPE => {
                if vec_iter.next() != Some(1) {
                    return None;
                }
                proprietary_vec.push(Concept2ResponseProprietary::GetWorkoutType(
                    vec_iter.next().unwrap(),
                ))
            }
            _ => {
                return None;
            }
        }
    }
    Some(Concept2Response::ProprietaryCommand(proprietary_vec))
}

fn parse_c2r<'a, T>(iter: &mut T) -> Option<Concept2Response>
where
    T: Iterator<Item = &'a u8>,
{
    let status = iter.next();
    let identifier = iter.next();
    let bytes = iter.next();
    match (status, identifier, bytes) {
        (Some(s), Some(i), Some(b)) => {
            let data: Vec<u8> = iter.take(usize::from(*b)).map(|&x| x).collect();
            if data.len() == usize::from(*b) {
                ResponseFrame {
                    _status: *s,
                    identifier: *i,
                    bytes: *b,
                    data: data,
                }
                .parse()
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_helper<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> Option<Vec<Concept2Response>> {
    let mut result = vec![];
    while let Some(c2r) = parse_c2r(iter) {
        result.push(c2r);
    }
    Some(result)
}

fn checksum_iter<'a>(iter: impl Iterator<Item = &'a u8>) -> u8 {
    iter.fold(0, |acc, &x| x ^ acc)
}

fn unpack_bytes(v: &Vec<u8>) -> Vec<u8> {
    let mut vec_iter = v.iter();
    // Skipping the report number and the start flag.
    let mut result: Vec<u8> = vec_iter.by_ref().take(2).cloned().collect();
    while let Some(x) = vec_iter.next() {
        match x {
            0xf2 => {
                result.push(0xf2);
                return result;
            }
            0xf3 => match vec_iter.next() {
                Some(&0x00) => result.push(0xf0),
                Some(&0x01) => result.push(0xf1),
                Some(&0x02) => result.push(0xf2),
                Some(&0x03) => result.push(0xf3),
                _ => {
                    result.push(0xf3);
                    return result;
                }
            },
            x => result.push(*x),
        }
    }
    result
}

pub fn parse_vec(v: &Vec<u8>) -> Option<Vec<Concept2Response>> {
    let unpacked_vec: Vec<u8> = unpack_bytes(v);
    let start_flag = unpacked_vec.iter().skip(1).next();
    let end_flag = unpacked_vec.iter().rev().next();
    let checksum = unpacked_vec.iter().rev().skip(1).next();
    let length = unpacked_vec.len();
    let actual_checksum = checksum_iter(unpacked_vec.iter().skip(2).take(length - 4));
    match (
        start_flag,
        end_flag,
        checksum.map(|x| *x == actual_checksum),
    ) {
        (Some(&consts::CSAFE_START_FLAG), Some(&consts::CSAFE_STOP_FLAG), Some(true)) => {
            parse_helper(&mut v.iter().skip(2).take(length - 4))
        }
        _ => None,
    }
}

mod tests {
    #[test]
    fn test_parse_get_user_id() {
        let v: Vec<u8> = vec![
            0x1, 0xf1, 0x81, 0x92, 0x5, 0x30, 0x30, 0x30, 0x30, 0x30, 0x26, 0xf2,
        ];
        assert_eq!(
            Some(vec![super::Concept2Response::GetUserID(String::from(
                "00000"
            ))]),
            super::parse_vec(&v)
        );
    }

    #[test]
    fn test_parse_get_serial_number() {
        let v: Vec<u8> = vec![
            0x1, 0xf1, 0x81, 0x94, 0x9, 0x34, 0x33, 0x30, 0x32, 0x32, 0x38, 0x35, 0x32, 0x35, 0x21,
            0xf2,
        ];
        assert_eq!(
            Some(vec![super::Concept2Response::GetSerialNumber(
                String::from("430228525")
            )]),
            super::parse_vec(&v)
        );
    }

    #[test]
    fn test_parse_get_odometer() {
        let v: Vec<u8> = vec![
            0x1, 0xf1, 0x81, 0x9b, 0x5, 0xf4, 0x24, 0x21, 0x0, 0x24, 0xca, 0xf2,
        ];
        assert_eq!(
            Some(vec![super::Concept2Response::GetOdometer(2172148, 0x24)]),
            super::parse_vec(&v)
        );
    }

    #[test]
    fn test_parse_get_work_distance() {
        let v: Vec<u8> = vec![
            0x1, 0xf1, 0x1, 0x1a, 0x7, 0xa3, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, 0xba, 0xf2,
        ];
        assert_eq!(
            Some(vec![super::Concept2Response::ProprietaryCommand(vec![
                super::Concept2ResponseProprietary::GetWorkDistance(0, 0)
            ])]),
            super::parse_vec(&v)
        );
    }

    #[test]
    fn test_parse_get_work_distance_and_workout_type() {
        let v: Vec<u8> = vec![
            0x1, 0xf1, 0x81, 0x1a, 0xa, 0xa3, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, 0x89, 0x1, 0x8, 0xb7,
            0xf2,
        ];
        assert_eq!(
            Some(vec![super::Concept2Response::ProprietaryCommand(vec![
                super::Concept2ResponseProprietary::GetWorkDistance(0, 0),
                super::Concept2ResponseProprietary::GetWorkoutType(8)
            ])]),
            super::parse_vec(&v)
        );
    }
}
