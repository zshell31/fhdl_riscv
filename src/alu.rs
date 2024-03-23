use ferrum_hdl::prelude::*;

use crate::{
    decoder::InstrR,
    types::{RVF3, RVF7, RVOP},
};

pub type Operand = U<32>;

#[derive(Debug, Default, Clone, Copy, SignalValue, BitPack, Traceable)]
pub enum Op {
    #[default]
    ADD = 0,
    OR = 1,
    SRL = 2,
    SLTU = 3,
    SUB = 4,
}

pub fn alu(src_a: Operand, src_b: Operand, op: Op) -> (Operand, Bit) {
    let result = match op {
        Op::ADD => src_a + src_b,
        Op::OR => src_a | src_b,
        Op::SRL => src_a >> Idx::<32>::cast_from(src_b),
        Op::SLTU => (src_a < src_b).cast(),
        Op::SUB => src_a - src_b,
    };
    let zero = result == 0;

    (result, zero)
}

#[derive(Debug, Default, Clone, Copy, SignalValue, Traceable)]
pub struct AluControl {
    pub pc_src: Bit,
    pub alu_src: Bit,
    pub wd_src: Bit,
    pub reg_write: Bit,
    pub op: Op,
}

impl AluControl {
    pub fn init(instr: InstrR, alu_zero: Bit) -> Self {
        let mut c = Self::default();
        let mut branch = false;
        let mut cond_zero = false;

        match (instr.f7, instr.f3, instr.code) {
            // (0, 0, 51)
            (RVF7::ADD, RVF3::ADD, RVOP::ADD) => {
                c.reg_write = true;
                c.op = Op::ADD;
            }
            // (0, 6, 51)
            (RVF7::OR, RVF3::OR, RVOP::OR) => {
                c.reg_write = true;
                c.op = Op::OR;
            }
            // (0, 5, 51)
            (RVF7::SRL, RVF3::SRL, RVOP::SRL) => {
                c.reg_write = true;
                c.op = Op::SRL;
            }
            // (0, 3, 51)
            (RVF7::SLTU, RVF3::SLTU, RVOP::SLTU) => {
                c.reg_write = true;
                c.op = Op::SLTU;
            }
            // (32, 0, 51)
            (RVF7::SUB, RVF3::SUB, RVOP::SUB) => {
                c.reg_write = true;
                c.op = Op::SUB;
            }
            // (_, 0, 19)
            (_, RVF3::ADDI, RVOP::ADDI) => {
                c.reg_write = true;
                c.alu_src = true;
                c.op = Op::ADD;
            }
            // (_, _, 55)
            (_, _, RVOP::LUI) => {
                c.reg_write = true;
                c.wd_src = true;
            }
            // (_, 0, 99)
            (_, RVF3::BEQ, RVOP::BEQ) => {
                branch = true;
                cond_zero = true;
                c.reg_write = true;
                c.wd_src = true;
                c.op = Op::SUB;
            }
            // (_, 1, 99)
            (_, RVF3::BNE, RVOP::BNE) => {
                branch = true;
                c.reg_write = true;
                c.wd_src = true;
                c.op = Op::SUB;
            }
            _ => {}
        }

        c.pc_src = branch && (alu_zero == cond_zero);

        c
    }
}
