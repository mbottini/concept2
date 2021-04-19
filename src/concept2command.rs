pub trait Concept2Command {
    fn to_vec(&self) -> Vec<u8>;
}

pub struct GetStatus;

impl Concept2Command for GetStatus {
    fn to_vec(&self) -> Vec<u8> {
        vec![0x80]
    }
}

pub struct GetVersion;

impl Concept2Command for GetVersion {
    fn to_vec(&self) -> Vec<u8> {
        vec![0x91]
    }
}

