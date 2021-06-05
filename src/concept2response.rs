use crate::consts;

#[derive(Debug, PartialEq, Eq)]
pub enum Concept2Response {
    GetStatus,
    GetVersion,
    GetUserID(String),
    GetSerialNumber(String),
}

pub struct ResponseFrame {
    status: u8,
    identifier: u8,
    bytes: u8,
    data: Vec<u8>,
}

impl ResponseFrame {
    pub fn parse(self) -> Option<Concept2Response> {
        match self.identifier {
            consts::CsafeCommands::GetUserID => {
                assert!(self.bytes == 5);
                Some(Concept2Response::GetUserID(
                    String::from_utf8(self.data).expect("parse error"),
                ))
            }
            consts::CsafeCommands::GetSerialNumber => {
                assert!(self.bytes == 9);
                Some(Concept2Response::GetSerialNumber(
                    String::from_utf8(self.data).expect("parse error"),
                ))
            }
            _ => None,
        }
    }
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
                    status: *s,
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

fn parse_vec(v: &Vec<u8>) -> Option<Vec<Concept2Response>> {
    let start_flag = v.iter().skip(1).next();
    let end_flag = v.iter().rev().next();
    let checksum = v.iter().rev().skip(1).next();
    let length = v.len();
    let actual_checksum = checksum_iter(v.iter().skip(2).take(length - 4));
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
}
