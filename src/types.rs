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
    ADD  = 0b0110011;
);

pub type OpCode = RVOP::Ty;

decl_consts!(
    RVF3 = U<3>;
    ADDI = 0b000;
    BEQ  = 0b000;
);

pub type Funct3 = RVF3::Ty;

decl_consts!(
    RVF7 = U<7>;
    ADD  = 0b0000000;
);

pub type Funct7 = RVF7::Ty;
