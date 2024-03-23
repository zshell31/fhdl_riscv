#![allow(incomplete_features)]
#![allow(clippy::let_and_return)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![feature(register_tool)]
#![register_tool(fhdl_tool)]

pub mod alu;
pub mod decoder;
pub mod types;

use alu::AluControl;
use decoder::Decoded;
use ferrum_hdl::prelude::*;
use types::Instr;

pub type Dom = TD4;

pub fn top(instr: Instr) -> AluControl {
    let d = Decoded::decode(&instr);
    AluControl::init(d.decoded, false)
}
