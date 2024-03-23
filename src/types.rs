use ferrum_hdl::prelude::*;

pub type Reg = U<5>;
pub type Instr = U<32>;

macro_rules! decl_consts {
    ($ty:ident = $inner:ty; $( $name:ident = $val:literal );* $(;)?) => {
        #[allow(non_snake_case)]
        pub mod $ty {
            use super::*;

            pub type Ty = $inner;

            $(
                pub const $name: Ty = <Ty>::from($val);
            )*
        }
    };
}

decl_consts!(
    RVOP = U<7>;
    ADDI = 0b0010011;
    BEQ  = 0b1100011;
    LUI  = 0b0110111;
    BNE  = 0b1100011;
    ADD  = 0b0110011;
    OR   = 0b0110011;
    SRL  = 0b0110011;
    SLTU = 0b0110011;
    SUB  = 0b0110011;
);

pub type OpCode = RVOP::Ty;

decl_consts!(
    RVF3 = U<3>;
    ADDI = 0b000;
    BEQ  = 0b000;
    BNE  = 0b001;
    ADD  = 0b000;
    OR   = 0b110;
    SRL  = 0b101;
    SLTU = 0b011;
    SUB  = 0b000;
);

pub type Funct3 = RVF3::Ty;

decl_consts!(
    RVF7 = U<7>;
    ADD  = 0b0000000;
    OR   = 0b0000000;
    SRL  = 0b0000000;
    SLTU = 0b0000000;
    SUB  = 0b0100000;
);

pub type Funct7 = RVF7::Ty;
