#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

fn main() {
    hprintln!("Hello, world!");
}

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 113] = [default_handler; 113];

extern "C" fn default_handler() {
    cortex_m::asm::bkpt();
}
