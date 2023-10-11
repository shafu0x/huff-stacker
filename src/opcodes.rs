#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Opcode {
    pub name: &'static str,
    pub pops: usize,
    pub pushes: usize,
    pub sign: Option<&'static str>,
    pub output: &'static str,
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
            &_ => panic!("Unknown opcode: {}", name),
        }
    }
}

macro_rules! define_opcode {
    ($name:ident, $name_str:expr, $pops:expr, $pushes:expr, $sign:expr, $output:expr) => {
        pub const $name: Opcode = Opcode {
            name: $name_str,
            pops: $pops,
            pushes: $pushes,
            sign: $sign,
            output: $output,
        };
    };
}

define_opcode!(STOP, "stop", 0, 0, None, "");
define_opcode!(ADD, "add", 2, 1, Some("+"), "");
define_opcode!(MUL, "mul", 2, 1, Some("*"), "");
define_opcode!(SUB, "sub", 2, 1, Some("-"), "");
define_opcode!(DIV, "div", 2, 1, Some("/"), "");
define_opcode!(SDIV, "sdiv", 2, 1, Some("/"), "");
define_opcode!(MOD, "mod", 2, 1, Some("%"), "");
define_opcode!(SMOD, "smod", 2, 1, Some("%"), "");
define_opcode!(ADDMOD, "addmod", 3, 1, None, "");
define_opcode!(MULMOD, "mulmod", 3, 1, None, "");
define_opcode!(EXP, "exp", 2, 1, Some("**"), "");
define_opcode!(SIGNEXTEND, "signextend", 2, 1, None, "");
define_opcode!(LT, "lt", 2, 1, Some("<"), "");
define_opcode!(GT, "gt", 2, 1, Some(">"), "");
define_opcode!(SLT, "slt", 2, 1, Some("<"), "");
define_opcode!(SGT, "sgt", 2, 1, Some(">"), "");
define_opcode!(EQ, "eq", 2, 1, Some("=="), "");
define_opcode!(ISZERO, "iszero", 1, 1, None, "");
define_opcode!(AND, "and", 2, 1, Some("&"), "");
define_opcode!(OR, "or", 2, 1, Some("|"), "");
define_opcode!(XOR, "xor", 2, 1, Some("^"), "");
define_opcode!(NOT, "not", 1, 1, None, "");
define_opcode!(BYTE, "byte", 2, 1, None, "");
define_opcode!(SHL, "shl", 2, 1, Some("<<"), "");
define_opcode!(SHR, "shr", 2, 1, Some(">>"), "");
define_opcode!(SAR, "sar", 2, 1, Some(">>"), "");
define_opcode!(SHA3, "sha3", 2, 1, None, "hash");
define_opcode!(ADDRESS, "address", 0, 1, None, "");
define_opcode!(BALANCE, "balance", 1, 1, None, "");
define_opcode!(ORIGIN, "origin", 0, 1, None, "");
define_opcode!(CALLER, "caller", 0, 1, None, "");
define_opcode!(CALLVALUE, "callvalue", 0, 1, None, "");
define_opcode!(CALLDATALOAD, "calldataload", 1, 1, None, "");
define_opcode!(CALLDATASIZE, "calldatasize", 0, 1, None, "");
define_opcode!(CALLDATACOPY, "calldatacopy", 3, 0, None, "");
define_opcode!(CODESIZE, "codesize", 0, 1, None, "");
define_opcode!(CODECOPY, "codecopy", 3, 0, None, "");
define_opcode!(GASPRICE, "gasprice", 0, 1, None, "");
define_opcode!(EXTCODESIZE, "extcodesize", 1, 1, None, "");
define_opcode!(EXTCODECOPY, "extcodecopy", 4, 0, None, "");
define_opcode!(RETURNDATASIZE, "returndatasize", 0, 1, None, "");
define_opcode!(RETURNDATACOPY, "returndatacopy", 3, 0, None, "");
define_opcode!(EXTCODEHASH, "extcodehash", 1, 1, None, "");
define_opcode!(BLOCKHASH, "blockhash", 1, 1, None, "");
define_opcode!(COINBASE, "coinbase", 0, 1, None, "");
define_opcode!(TIMESTAMP, "timestamp", 0, 1, None, "");
define_opcode!(NUMBER, "number", 0, 1, None, "");
define_opcode!(PREVRANDAO, "prevrandao", 0, 1, None, "");
define_opcode!(GASLIMIT, "gaslimit", 0, 1, None, "");
define_opcode!(CHAINID, "chainid", 0, 1, None, "");
define_opcode!(SELFBALANCE, "selfbalance", 0, 1, None, "");
define_opcode!(BASEFEE, "basefee", 0, 1, None, "");
define_opcode!(POP, "pop", 1, 0, None, "");
define_opcode!(MLOAD, "mload", 1, 1, None, "");
define_opcode!(MSTORE, "mstore", 2, 0, None, "");
define_opcode!(MSTORE8, "mstore8", 2, 0, None, "");
define_opcode!(SLOAD, "sload", 1, 1, None, "");
define_opcode!(SSTORE, "sstore", 2, 0, None, "");
define_opcode!(JUMP, "jump", 1, 0, None, "");
define_opcode!(JUMPI, "jumpi", 2, 0, None, "");
define_opcode!(PC, "pc", 0, 1, None, "");
define_opcode!(MSIZE, "msize", 0, 1, None, "");
define_opcode!(GAS, "gas", 0, 1, None, "");
define_opcode!(JUMPDEST, "jumpdest", 0, 0, None, "");
define_opcode!(LOG0, "log0", 2, 0, None, "");
define_opcode!(LOG1, "log1", 3, 0, None, "");
define_opcode!(LOG2, "log2", 4, 0, None, "");
define_opcode!(LOG3, "log3", 5, 0, None, "");
define_opcode!(LOG4, "log4", 6, 0, None, "");
define_opcode!(CREATE, "create", 3, 1, None, "address");
define_opcode!(CALL, "call", 7, 1, None, "success");
define_opcode!(CALLCODE, "callcode", 7, 1, None, "");
define_opcode!(RETURN, "return", 2, 0, None, "");
define_opcode!(DELEGATECALL, "delegatecall", 6, 1, None, "success");
define_opcode!(CREATE2, "create2", 4, 1, None, "address");
define_opcode!(STATICCALL, "staticcall", 6, 1, None, "success");
define_opcode!(REVERT, "revert", 2, 0, None, "");
define_opcode!(INVALID, "invalid", 0, 0, None, "");
define_opcode!(SELFDESTRUCT, "selfdestruct", 1, 0, None, "");
define_opcode!(UNKNOWN, "unknown", 0, 0, None, "");
