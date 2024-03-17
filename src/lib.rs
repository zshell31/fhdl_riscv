#![allow(incomplete_features)]
#![allow(clippy::let_and_return)]
#![feature(generic_const_exprs)]

use ferrum_hdl::prelude::*;

pub type Dom = TD4;

pub fn top(
    clk_in: &Clock<Dom>,
    rst: &Reset<Dom>,
    _clk_divide: &Signal<Dom, U<4>>,
    _clk_en: &Enable<Dom>,
    _reg_addr: &Signal<Dom, U<5>>,
) -> (Signal<Dom, Bit>, Signal<Dom, U<32>>) {
    let clk = reg0(clk_in, rst, |clk: Bit| !clk);
    let reg_data = reg0(clk_in, rst, |reg_data| reg_data + 1);

    (clk, reg_data)
}
