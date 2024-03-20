use ferrum_hdl::prelude::*;

pub type Operand = U<32>;

#[derive(Debug, Clone, Copy, SignalValue, BitPack, Traceable)]
#[non_exhaustive]
pub enum Op {
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
