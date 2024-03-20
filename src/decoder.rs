use ferrum_hdl::prelude::*;

use crate::types::{Funct3, Funct7, Instr, OpCode, Reg};

pub type Imm = S<32>;

#[derive(Debug, Clone, SignalValue, BitPack, Traceable)]
pub struct InstrR {
    // 31:25
    pub f7: Funct7,
    // 24:20
    pub rs2: Reg,
    // 19:15
    pub rs1: Reg,
    // 14:12
    pub f3: Funct3,
    // 11:7
    pub rd: Reg,
    // 6:0
    pub op_code: OpCode,
}

#[derive(Debug, Clone, SignalValue, Traceable)]
pub struct Decoded {
    pub decoded: InstrR,
    pub imm_i: Imm,
    pub imm_b: Imm,
    pub imm_u: Imm,
}

impl Decoded {
    pub fn decode(instr: &Instr) -> Self {
        let decoded = instr.repack();

        let imm_i = bits!(instr[31:20]).cast::<S<12>>().sign_extend();
        let imm_b = bits!(instr[31, 7, 30:25, 11:8])
            .repack::<S<_>>()
            .sign_extend();
        let imm_u = bits!(instr[31:12, L:12]).repack();

        Self {
            decoded,
            imm_i,
            imm_b,
            imm_u,
        }
    }
}
