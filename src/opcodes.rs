#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Opcode {
    pub name: &'static str,
    pub pops: usize,
    pub pushes: usize,
    pub sign: Option<&'static str>,
}

impl Opcode {
    pub fn from_string(name: &str) -> Opcode {
        match name {
            "stop" => STOP,
            "add" => ADD,
            "mul" => MUL,
            "sub" => SUB,
            "div" => DIV,
            "sdiv" => SDIV,
            "mod" => MOD,
            "smod" => SMOD,
            "addmod" => ADDMOD,
            "mulmod" => MULMOD,
            "exp" => EXP,
            "signextend" => SIGNEXTEND,
            "lt" => LT,
            "gt" => GT,
            "slt" => SLT,
            "sgt" => SGT,
            "eq" => EQ,
            "iszero" => ISZERO,
            "and" => AND,
            "or" => OR,
            "xor" => XOR,
            "not" => NOT,
            "byte" => BYTE,
            "shl" => SHL,
            "shr" => SHR,
            "sar" => SAR,
            "sha3" => SHA3,
            "address" => ADDRESS,
            "balance" => BALANCE,
            "origin" => ORIGIN,
            "caller" => CALLER,
            "callvalue" => CALLVALUE,
            "calldataload" => CALLDATALOAD,
            "calldatasize" => CALLDATASIZE,
            "calldatacopy" => CALLDATACOPY,
            "codesize" => CODESIZE,
            "codecopy" => CODECOPY,
            "gasprice" => GASPRICE,
            "extcodesize" => EXTCODESIZE,
            "extcodecopy" => EXTCODECOPY,
            "returndatasize" => RETURNDATASIZE,
            "returndatacopy" => RETURNDATACOPY,
            "extcodehash" => EXTCODEHASH,
            "blockhash" => BLOCKHASH,
            "coinbase" => COINBASE,
            "timestamp" => TIMESTAMP,
            "number" => NUMBER,
            "prevrandao" => PREVRANDAO,
            "gaslimit" => GASLIMIT,
            "chainid" => CHAINID,
            "selfbalance" => SELFBALANCE,
            "basefee" => BASEFEE,
            "pop" => POP,
            "mload" => MLOAD,
            "mstore" => MSTORE,
            "mstore8" => MSTORE8,
            "sload" => SLOAD,
            "sstore" => SSTORE,
            "jump" => JUMP,
            "jumpi" => JUMPI,
            "pc" => PC,
            "msize" => MSIZE,
            "gas" => GAS,
            "jumpdest" => JUMPDEST,
            "log0" => LOG0,
            "log1" => LOG1,
            "log2" => LOG2,
            "log3" => LOG3,
            "log4" => LOG4,
            "create" => CREATE,
            "call" => CALL,
            "callcode" => CALLCODE,
            "return" => RETURN,
            "delegatecall" => DELEGATECALL,
            "create2" => CREATE2,
            "staticcall" => STATICCALL,
            "revert" => REVERT,
            "invalid" => INVALID,
            "selfdestruct" => SELFDESTRUCT,
            &_ => INVALID,
        }
    }
}

pub const STOP: Opcode = Opcode {
    name: "stop",
    pops: 0,
    pushes: 0,
    sign: None,
};
pub const ADD: Opcode = Opcode {
    name: "add",
    pops: 2,
    pushes: 1,
    sign: Some("+"),
};
pub const MUL: Opcode = Opcode {
    name: "mul",
    pops: 2,
    pushes: 1,
    sign: Some("*"),
};
pub const SUB: Opcode = Opcode {
    name: "sub",
    pops: 2,
    pushes: 1,
    sign: Some("-"),
};
pub const DIV: Opcode = Opcode {
    name: "div",
    pops: 2,
    pushes: 1,
    sign: Some("/"),
};
pub const SDIV: Opcode = Opcode {
    name: "sdiv",
    pops: 2,
    pushes: 1,
    sign: Some("/"),
};
pub const MOD: Opcode = Opcode {
    name: "mod",
    pops: 2,
    pushes: 1,
    sign: Some("%"),
};
pub const SMOD: Opcode = Opcode {
    name: "smod",
    pops: 2,
    pushes: 1,
    sign: Some("%"),
};
pub const ADDMOD: Opcode = Opcode {
    name: "addmod",
    pops: 3,
    pushes: 1,
    sign: None,
};
pub const MULMOD: Opcode = Opcode {
    name: "mulmod",
    pops: 3,
    pushes: 1,
    sign: None,
};
pub const EXP: Opcode = Opcode {
    name: "exp",
    pops: 2,
    pushes: 1,
    sign: Some("**"),
};
pub const SIGNEXTEND: Opcode = Opcode {
    name: "signextend",
    pops: 2,
    pushes: 1,
    sign: None,
};
pub const LT: Opcode = Opcode {
    name: "lt",
    pops: 2,
    pushes: 1,
    sign: Some("<"),
};
pub const GT: Opcode = Opcode {
    name: "gt",
    pops: 2,
    pushes: 1,
    sign: Some(">"),
};
pub const SLT: Opcode = Opcode {
    name: "slt",
    pops: 2,
    pushes: 1,
    sign: Some("<"),
};
pub const SGT: Opcode = Opcode {
    name: "sgt",
    pops: 2,
    pushes: 1,
    sign: Some(">"),
};
pub const EQ: Opcode = Opcode {
    name: "eq",
    pops: 2,
    pushes: 1,
    sign: Some("=="),
};
pub const ISZERO: Opcode = Opcode {
    name: "iszero",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const AND: Opcode = Opcode {
    name: "and",
    pops: 2,
    pushes: 1,
    sign: Some("&"),
};
pub const OR: Opcode = Opcode {
    name: "or",
    pops: 2,
    pushes: 1,
    sign: Some("|"),
};
pub const XOR: Opcode = Opcode {
    name: "xor",
    pops: 2,
    pushes: 1,
    sign: Some("^"),
};
pub const NOT: Opcode = Opcode {
    name: "not",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const BYTE: Opcode = Opcode {
    name: "byte",
    pops: 2,
    pushes: 1,
    sign: None,
};
pub const SHL: Opcode = Opcode {
    name: "shl",
    pops: 2,
    pushes: 1,
    sign: Some("<<"),
};
pub const SHR: Opcode = Opcode {
    name: "shr",
    pops: 2,
    pushes: 1,
    sign: Some(">>"),
};
pub const SAR: Opcode = Opcode {
    name: "sar",
    pops: 2,
    pushes: 1,
    sign: Some(">>"),
};
pub const SHA3: Opcode = Opcode {
    name: "sha3",
    pops: 2,
    pushes: 1,
    sign: None,
};
pub const ADDRESS: Opcode = Opcode {
    name: "address",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const BALANCE: Opcode = Opcode {
    name: "balance",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const ORIGIN: Opcode = Opcode {
    name: "origin",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const CALLER: Opcode = Opcode {
    name: "caller",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const CALLVALUE: Opcode = Opcode {
    name: "callvalue",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const CALLDATALOAD: Opcode = Opcode {
    name: "calldataload",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const CALLDATASIZE: Opcode = Opcode {
    name: "calldatasize",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const CALLDATACOPY: Opcode = Opcode {
    name: "calldatacopy",
    pops: 3,
    pushes: 0,
    sign: None,
};
pub const CODESIZE: Opcode = Opcode {
    name: "codesize",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const CODECOPY: Opcode = Opcode {
    name: "codecopy",
    pops: 3,
    pushes: 0,
    sign: None,
};
pub const GASPRICE: Opcode = Opcode {
    name: "gasprice",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const EXTCODESIZE: Opcode = Opcode {
    name: "extcodesize",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const EXTCODECOPY: Opcode = Opcode {
    name: "extcodecopy",
    pops: 4,
    pushes: 0,
    sign: None,
};
pub const RETURNDATASIZE: Opcode = Opcode {
    name: "returndatasize",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const RETURNDATACOPY: Opcode = Opcode {
    name: "returndatacopy",
    pops: 3,
    pushes: 0,
    sign: None,
};
pub const EXTCODEHASH: Opcode = Opcode {
    name: "extcodehash",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const BLOCKHASH: Opcode = Opcode {
    name: "blockhash",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const COINBASE: Opcode = Opcode {
    name: "coinbase",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const TIMESTAMP: Opcode = Opcode {
    name: "timestamp",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const NUMBER: Opcode = Opcode {
    name: "number",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const PREVRANDAO: Opcode = Opcode {
    name: "prevrandao",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const GASLIMIT: Opcode = Opcode {
    name: "gaslimit",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const CHAINID: Opcode = Opcode {
    name: "chainid",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const SELFBALANCE: Opcode = Opcode {
    name: "selfbalance",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const BASEFEE: Opcode = Opcode {
    name: "basefee",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const POP: Opcode = Opcode {
    name: "pop",
    pops: 1,
    pushes: 0,
    sign: None,
};
pub const MLOAD: Opcode = Opcode {
    name: "mload",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const MSTORE: Opcode = Opcode {
    name: "mstore",
    pops: 2,
    pushes: 0,
    sign: None,
};
pub const MSTORE8: Opcode = Opcode {
    name: "mstore8",
    pops: 2,
    pushes: 0,
    sign: None,
};
pub const SLOAD: Opcode = Opcode {
    name: "sload",
    pops: 1,
    pushes: 1,
    sign: None,
};
pub const SSTORE: Opcode = Opcode {
    name: "sstore",
    pops: 2,
    pushes: 0,
    sign: None,
};
pub const JUMP: Opcode = Opcode {
    name: "jump",
    pops: 1,
    pushes: 0,
    sign: None,
};
pub const JUMPI: Opcode = Opcode {
    name: "jumpi",
    pops: 2,
    pushes: 0,
    sign: None,
};
pub const PC: Opcode = Opcode {
    name: "PC",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const MSIZE: Opcode = Opcode {
    name: "MSIZE",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const GAS: Opcode = Opcode {
    name: "GAS",
    pops: 0,
    pushes: 1,
    sign: None,
};
pub const JUMPDEST: Opcode = Opcode {
    name: "jumpdest",
    pops: 0,
    pushes: 0,
    sign: None,
};
pub const LOG0: Opcode = Opcode {
    name: "log0",
    pops: 2,
    pushes: 0,
    sign: None,
};
pub const LOG1: Opcode = Opcode {
    name: "log1",
    pops: 3,
    pushes: 0,
    sign: None,
};
pub const LOG2: Opcode = Opcode {
    name: "log2",
    pops: 4,
    pushes: 0,
    sign: None,
};
pub const LOG3: Opcode = Opcode {
    name: "log3",
    pops: 5,
    pushes: 0,
    sign: None,
};
pub const LOG4: Opcode = Opcode {
    name: "log4",
    pops: 6,
    pushes: 0,
    sign: None,
};
pub const CREATE: Opcode = Opcode {
    name: "create",
    pops: 3,
    pushes: 1,
    sign: None,
};
pub const CALL: Opcode = Opcode {
    name: "call",
    pops: 7,
    pushes: 1,
    sign: None,
};
pub const CALLCODE: Opcode = Opcode {
    name: "callcode",
    pops: 7,
    pushes: 1,
    sign: None,
};
pub const RETURN: Opcode = Opcode {
    name: "return",
    pops: 2,
    pushes: 0,
    sign: None,
};
pub const DELEGATECALL: Opcode = Opcode {
    name: "delegatecall",
    pops: 6,
    pushes: 1,
    sign: None,
};
pub const CREATE2: Opcode = Opcode {
    name: "create2",
    pops: 4,
    pushes: 1,
    sign: None,
};
pub const STATICCALL: Opcode = Opcode {
    name: "staticcall",
    pops: 6,
    pushes: 1,
    sign: None,
};
pub const REVERT: Opcode = Opcode {
    name: "revert",
    pops: 2,
    pushes: 0,
    sign: None,
};
pub const INVALID: Opcode = Opcode {
    name: "invalid",
    pops: 0,
    pushes: 0,
    sign: None,
};
pub const SELFDESTRUCT: Opcode = Opcode {
    name: "selfdestruct",
    pops: 1,
    pushes: 0,
    sign: None,
};
pub const UNKNOWN: Opcode = Opcode {
    name: "unknown",
    pops: 0,
    pushes: 0,
    sign: None,
};
