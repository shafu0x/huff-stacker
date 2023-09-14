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

pub const SSTORE: Opcode = Opcode {
    name: "sstore",
    pops: 2,
    pushes: 0,
};
pub const MSTORE: Opcode = Opcode {
    name: "mstore",
    pops: 2,
    pushes: 0,
};
pub const MLOAD: Opcode = Opcode {
    name: "mload",
    pops: 1,
    pushes: 1,
};
pub const CALLDATALOAD: Opcode = Opcode {
    name: "calldataload",
    pops: 1,
    pushes: 1,
};
