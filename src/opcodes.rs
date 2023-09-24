#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Opcode {
    pub name: &'static str,
    pub pops: usize,
    pub pushes: usize,
    pub swap: usize,
    pub dupe: usize,
    pub sign: Option<&'static str>,
}

impl Opcode {
    pub fn from_string(name: &str) -> Opcode {
        match name {
            "stop" => STOP,
            "swap1" => SWAP1,
            "swap2" => SWAP2,
            "swap3" => SWAP3,
            "swap4" => SWAP4,
            "swap5" => SWAP5,
            "swap6" => SWAP6,
            "swap7" => SWAP7,
            "swap8" => SWAP8,
            "swap9" => SWAP9,
            "swap10" => SWAP10,
            "swap11" => SWAP11,
            "swap12" => SWAP12,
            "swap13" => SWAP13,
            "swap14" => SWAP14,
            "swap15" => SWAP15,
            "swap16" => SWAP16,
            "dup1" => DUP1,
            "dup2" => DUP2,
            "dup3" => DUP3,
            "dup4" => DUP4,
            "dup5" => DUP5,
            "dup6" => DUP6,
            "dup7" => DUP7,
            "dup8" => DUP8,
            "dup9" => DUP9,
            "dup10" => DUP10,
            "dup11" => DUP11,
            "dup12" => DUP12,
            "dup13" => DUP13,
            "dup14" => DUP14,
            "dup15" => DUP15,
            "dup16" => DUP16,
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
            &_ => panic!("Unknown opcode: {}", name),
        }
    }
}

macro_rules! define_opcode {
    ($name:ident, $name_str:expr, $pops:expr, $pushes:expr, $swap:expr, $dupe:expr, $sign:expr) => {
        pub const $name: Opcode = Opcode {
            name: $name_str,
            pops: $pops,
            pushes: $pushes,
            swap: $swap,
            dupe: $dupe,
            sign: $sign,
        };
    };
}

define_opcode!(STOP, "stop", 0, 0, 0, 0, None);
define_opcode!(SWAP1, "SWAP1", 0, 0, 2, 0, None);
define_opcode!(SWAP2, "SWAP2", 0, 0, 3, 0, None);
define_opcode!(SWAP3, "SWAP3", 0, 0, 4, 0, None);
define_opcode!(SWAP4, "SWAP4", 0, 0, 5, 0, None);
define_opcode!(SWAP5, "SWAP5", 0, 0, 6, 0, None);
define_opcode!(SWAP6, "SWAP6", 0, 0, 7, 0, None);
define_opcode!(SWAP7, "SWAP7", 0, 0, 8, 0, None);
define_opcode!(SWAP8, "SWAP8", 0, 0, 9, 0, None);
define_opcode!(SWAP9, "SWAP9", 0, 0, 10, 0, None);
define_opcode!(SWAP10, "SWAP10", 0, 0, 11, 0, None);
define_opcode!(SWAP11, "SWAP11", 0, 0, 12, 0, None);
define_opcode!(SWAP12, "SWAP12", 0, 0, 13, 0, None);
define_opcode!(SWAP13, "SWAP13", 0, 0, 14, 0, None);
define_opcode!(SWAP14, "SWAP14", 0, 0, 15, 0, None);
define_opcode!(SWAP15, "SWAP15", 0, 0, 16, 0, None);
define_opcode!(SWAP16, "SWAP16", 0, 0, 17, 0, None);

define_opcode!(DUP1, "DUP1", 0, 0, 0, 1, None);
define_opcode!(DUP2, "DUP2", 0, 0, 0, 2, None);
define_opcode!(DUP3, "DUP3", 0, 0, 0, 3, None);
define_opcode!(DUP4, "DUP4", 0, 0, 0, 4, None);
define_opcode!(DUP5, "DUP5", 0, 0, 0, 5, None);
define_opcode!(DUP6, "DUP6", 0, 0, 0, 6, None);
define_opcode!(DUP7, "DUP7", 0, 0, 0, 7, None);
define_opcode!(DUP8, "DUP8", 0, 0, 0, 8, None);
define_opcode!(DUP9, "DUP9", 0, 0, 0, 9, None);
define_opcode!(DUP10, "DUP10", 0, 0, 0, 10, None);
define_opcode!(DUP11, "DUP11", 0, 0, 0, 11, None);
define_opcode!(DUP12, "DUP12", 0, 0, 0, 12, None);
define_opcode!(DUP13, "DUP13", 0, 0, 0, 13, None);
define_opcode!(DUP14, "DUP14", 0, 0, 0, 14, None);
define_opcode!(DUP15, "DUP15", 0, 0, 0, 15, None);
define_opcode!(DUP16, "DUP16", 0, 0, 0, 16, None);
define_opcode!(ADD, "add", 2, 1, 0, 0, Some("+"));
define_opcode!(MUL, "mul", 2, 1, 0, 0, Some("*"));
define_opcode!(SUB, "sub", 2, 1, 0, 0, Some("-"));
define_opcode!(DIV, "div", 2, 1, 0, 0, Some("/"));
define_opcode!(SDIV, "sdiv", 2, 1, 0, 0, Some("/"));
define_opcode!(MOD, "mod", 2, 1, 0, 0, Some("%"));
define_opcode!(SMOD, "smod", 2, 1, 0, 0, Some("%"));
define_opcode!(ADDMOD, "addmod", 3, 1, 0, 0, None);
define_opcode!(MULMOD, "mulmod", 3, 1, 0, 0, None);
define_opcode!(EXP, "exp", 2, 1, 0, 0, Some("**"));
define_opcode!(SIGNEXTEND, "signextend", 2, 1, 0, 0, None);
define_opcode!(LT, "lt", 2, 1, 0, 0, Some("<"));
define_opcode!(GT, "gt", 2, 1, 0, 0, Some(">"));
define_opcode!(SLT, "slt", 2, 1, 0, 0, Some("<"));
define_opcode!(SGT, "sgt", 2, 1, 0, 0, Some(">"));
define_opcode!(EQ, "eq", 2, 1, 0, 0, Some("=="));
define_opcode!(ISZERO, "iszero", 1, 1, 0, 0, None);
define_opcode!(AND, "and", 2, 1, 0, 0, Some("&"));
define_opcode!(OR, "or", 2, 1, 0, 0, Some("|"));
define_opcode!(XOR, "xor", 2, 1, 0, 0, Some("^"));
define_opcode!(NOT, "not", 1, 1, 0, 0, None);
define_opcode!(BYTE, "byte", 2, 1, 0, 0, None);
define_opcode!(SHL, "shl", 2, 1, 0, 0, Some("<<"));
define_opcode!(SHR, "shr", 2, 1, 0, 0, Some(">>"));
define_opcode!(SAR, "sar", 2, 1, 0, 0, Some(">>"));
define_opcode!(SHA3, "sha3", 2, 1, 0, 0, None);
define_opcode!(ADDRESS, "address", 0, 1, 0, 0, None);
define_opcode!(BALANCE, "balance", 1, 1, 0, 0, None);
define_opcode!(ORIGIN, "origin", 0, 1, 0, 0, None);
define_opcode!(CALLER, "caller", 0, 1, 0, 0, None);
define_opcode!(CALLVALUE, "callvalue", 0, 1, 0, 0, None);
define_opcode!(CALLDATALOAD, "calldataload", 1, 1, 0, 0, None);
define_opcode!(CALLDATASIZE, "calldatasize", 0, 1, 0, 0, None);
define_opcode!(CALLDATACOPY, "calldatacopy", 3, 0, 0, 0, None);
define_opcode!(CODESIZE, "codesize", 0, 1, 0, 0, None);
define_opcode!(CODECOPY, "codecopy", 3, 0, 0, 0, None);
define_opcode!(GASPRICE, "gasprice", 0, 1, 0, 0, None);
define_opcode!(EXTCODESIZE, "extcodesize", 1, 1, 0, 0, None);
define_opcode!(EXTCODECOPY, "extcodecopy", 4, 0, 0, 0, None);
define_opcode!(RETURNDATASIZE, "returndatasize", 0, 1, 0, 0, None);
define_opcode!(RETURNDATACOPY, "returndatacopy", 3, 0, 0, 0, None);
define_opcode!(EXTCODEHASH, "extcodehash", 1, 1, 0, 0, None);
define_opcode!(BLOCKHASH, "blockhash", 1, 1, 0, 0, None);
define_opcode!(COINBASE, "coinbase", 0, 1, 0, 0, None);
define_opcode!(TIMESTAMP, "timestamp", 0, 1, 0, 0, None);
define_opcode!(NUMBER, "number", 0, 1, 0, 0, None);
define_opcode!(PREVRANDAO, "prevrandao", 0, 1, 0, 0, None);
define_opcode!(GASLIMIT, "gaslimit", 0, 1, 0, 0, None);
define_opcode!(CHAINID, "chainid", 0, 1, 0, 0, None);
define_opcode!(SELFBALANCE, "selfbalance", 0, 1, 0, 0, None);
define_opcode!(BASEFEE, "basefee", 0, 1, 0, 0, None);
define_opcode!(POP, "pop", 1, 0, 0, 0, None);
define_opcode!(MLOAD, "mload", 1, 1, 0, 0, None);
define_opcode!(MSTORE, "mstore", 2, 0, 0, 0, None);
define_opcode!(MSTORE8, "mstore8", 2, 0, 0, 0, None);
define_opcode!(SLOAD, "sload", 1, 1, 0, 0, None);
define_opcode!(SSTORE, "sstore", 2, 0, 0, 0, None);
define_opcode!(JUMP, "jump", 1, 0, 0, 0, None);
define_opcode!(JUMPI, "jumpi", 2, 0, 0, 0, None);
define_opcode!(PC, "pc", 0, 1, 0, 0, None);
define_opcode!(MSIZE, "msize", 0, 1, 0, 0, None);
define_opcode!(GAS, "gas", 0, 1, 0, 0, None);
define_opcode!(JUMPDEST, "jumpdest", 0, 0, 0, 0, None);
define_opcode!(LOG0, "log0", 2, 0, 0, 0, None);
define_opcode!(LOG1, "log1", 3, 0, 0, 0, None);
define_opcode!(LOG2, "log2", 4, 0, 0, 0, None);
define_opcode!(LOG3, "log3", 5, 0, 0, 0, None);
define_opcode!(LOG4, "log4", 6, 0, 0, 0, None);
define_opcode!(CREATE, "create", 3, 1, 0, 0, None);
define_opcode!(CALL, "call", 7, 1, 0, 0, None);
define_opcode!(CALLCODE, "callcode", 7, 1, 0, 0, None);
define_opcode!(RETURN, "return", 2, 0, 0, 0, None);
define_opcode!(DELEGATECALL, "delegatecall", 6, 1, 0, 0, None);
define_opcode!(CREATE2, "create2", 4, 1, 0, 0, None);
define_opcode!(STATICCALL, "staticcall", 6, 1, 0, 0, None);
define_opcode!(REVERT, "revert", 2, 0, 0, 0, None);
define_opcode!(INVALID, "invalid", 0, 0, 0, 0, None);
define_opcode!(SELFDESTRUCT, "selfdestruct", 1, 0, 0, 0, None);
define_opcode!(UNKNOWN, "unknown", 0, 0, 0, 0, None);
