use crate::consts;

pub enum Concept2Response {
    GetStatus,
    GetVersion,
    GetSerialNumber(String),
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
            consts::CsafeCommands::GetSerialNumber => {
                assert!(self.bytes == 9);
                Some(Concept2Response::GetSerialNumber(String::from_utf8(self.data).expect("parse error")))
            },
            _ => None
        }
    }
}

fn parse_c2r<T>(iter: &mut T) -> Option<ResponseFrame> 
    where T: Iterator<Item=u8> {
    let status = iter.next();
    let identifier = iter.next();
    let bytes = iter.next();
    match (status, identifier, bytes) {
        (Some(s), Some(i), Some(b)) => {
            let data: Vec<u8> = iter.take(usize::from(b)).collect();
            if data.len() == usize::from(b) {
                Some(ResponseFrame {
                    status: s,
                    identifier: i,
                    bytes: b,
                    data: data,
                })
            } else {
                None
            }
        }
        _ => None
    }
}

fn parse_helper<T>(iter: &mut T) -> Option<Vec<ResponseFrame>> 
    where T: Iterator<Item=u8> {
    let mut result = vec![];
    while let Some(current_head) = iter.next() {
        if current_head == consts::CSAFE_STOP_FLAG {
            return Some(result);
        }
        let curr = parse_c2r(iter);
        match curr {
            Some(frame) => result.push(frame),
            None => return None,
        }
    }
    None
}

/*
fn parse_vec(v: &Vec<u8>) -> Option<Vec<Concept2Response>> {
    let mut vec_iter = v.iter();
    let report_num = vec_iter.next();
    let start_flag = vec_iter.next();
}
*/
/*
pub fn parseVec(Vec<u8> &v) -> Option<Concept2Response> {
}
*/