#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate tm4c129x;

use cortex_m::exception::Handlers as ExceptionHandlers;
use tm4c129x::interrupt::Handlers as InterruptHandlers;

fn main() {
    hprintln!("Hello, world!");

    cortex_m::interrupt::free(|cs| {
        let systick = tm4c129x::SYST.borrow(cs);
        let sysctl  = tm4c129x::SYSCTL.borrow(cs);
        let gpio_n  = tm4c129x::GPIO_PORTN.borrow(cs);

        systick.set_reload(systick.get_ticks_per_10ms() * 100);
        systick.enable_counter();
        systick.enable_interrupt();

        sysctl.rcgcgpio.write(|w| w.r12().bit(true));
        while !sysctl.prgpio.read().r12().bit() {}

        gpio_n.dir.write(|w| w.bits(0x02));
        gpio_n.den.write(|w| w.bits(0x02));
    });
}

extern fn sys_tick(_: cortex_m::exception::SysTick) {
    cortex_m::interrupt::free(|cs| {
        let gpio_n = tm4c129x::GPIO_PORTN.borrow(cs);
        gpio_n.data.modify(|r, w| w.bits(r.bits() ^ 0x02));
    })
}

#[used]
#[link_section = ".rodata.exceptions"]
pub static EXCEPTIONS: ExceptionHandlers = ExceptionHandlers {
    sys_tick: sys_tick,
    ..cortex_m::exception::DEFAULT_HANDLERS
};

#[used]
#[link_section = ".rodata.interrupts"]
pub static INTERRUPTS: InterruptHandlers = InterruptHandlers {
    ..tm4c129x::interrupt::DEFAULT_HANDLERS
};
