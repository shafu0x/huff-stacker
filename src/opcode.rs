
#[derive(PartialEq, Eq)]
pub struct Opcode {
    pub name: &'static str,
    pub pops: usize,
    pub pushes: usize,
}

impl Opcode {
    pub fn new(name: &'static str, pops: usize, pushes: usize) -> Opcode {
        Opcode {
            name: name,
            pops: pops,
            pushes: pushes,
        }
    }

    pub fn from_string(name: &str) -> Opcode {
        match name {
            "sstore" => SSTORE,
            "mstore" => MSTORE,
            "mload" => MLOAD,
            "calldataload" => CALLDATALOAD,
            &_ => todo!(),
        }
    }
}
