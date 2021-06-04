use crate::consts;

#[derive(Debug, PartialEq, Eq)]
pub enum Concept2Response {
    GetStatus,
    GetVersion,
    GetUserID(String),
}

pub struct ResponseFrame {
    status: u8,
    identifier: u8,
    bytes: u8,
    data: Vec<u8>
}

impl ResponseFrame {
    pub fn parse(self) -> Option<Concept2Response> {
        match self.identifier {
            consts::CsafeCommands::GetUserID => {
                assert!(self.bytes == 5);
                Some(Concept2Response::GetUserID(String::from_utf8(self.data).expect("parse error")))
            },
            _ => None
        }
    }
}

fn parse_c2r<'a, T>(iter: &mut T) -> Option<Concept2Response> 
    where T: Iterator<Item=&'a u8> {
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
                }.parse()
            } else {
                None
            }
        }
        _ => None
    }
}

fn parse_helper<'a>(iter: &mut impl Iterator<Item=&'a u8>) -> Option<Vec<Concept2Response>> {
    let mut result = vec![];
    while let Some(current_head) = iter.next() {
        if current_head == &consts::CSAFE_STOP_FLAG {
            return Some(result);
        }
        let curr = parse_c2r(iter);
        match curr {
            Some(c2r) => result.push(c2r),
            None => return None,
        }
    }
    None
}

fn parse_vec(v: &Vec<u8>) -> Option<Vec<Concept2Response>> {
    let mut vec_iter = v.iter();
    let _report_num = vec_iter.next();
    let start_flag = vec_iter.next();
    match start_flag {
        Some(&consts::CSAFE_START_FLAG) => parse_helper(&mut vec_iter),
        _ => None
    }
}

