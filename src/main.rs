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
        let gpio_a  = tm4c129x::GPIO_PORTA_AHB.borrow(cs);
        let gpio_n  = tm4c129x::GPIO_PORTN.borrow(cs);
        let uart0   = tm4c129x::UART0.borrow(cs);

        // Set up system timer
        systick.set_reload(systick.get_ticks_per_10ms() * 100);
        systick.enable_counter();
        systick.enable_interrupt();

        // Set up LEDs
        sysctl.rcgcgpio.modify(|_, w| w.r12().bit(true));
        while !sysctl.prgpio.read().r12().bit() {}

        gpio_n.dir.write(|w| w.dir().bits(0x02));
        gpio_n.den.write(|w| w.den().bits(0x02));

        // Set up UART0
        sysctl.rcgcgpio.modify(|_, w| w.r0().bit(true));
        while !sysctl.prgpio.read().r0().bit() {}

        gpio_a.dir.write(|w| w.dir().bits(0x02));
        gpio_a.den.write(|w| w.den().bits(0x03));
        gpio_a.afsel.write(|w| w.afsel().bits(0x3));
        gpio_a.pctl.write(|w| unsafe { w.pmc0().bits(1).pmc1().bits(1) });

        sysctl.rcgcuart.modify(|_, w| w.r0().bit(true));
        while !sysctl.pruart.read().r0().bit() {}

        let brd = /*uartclk*/16_000_000u32 * /*width(brdf)*/64 / (/*clkdiv*/16 * /*baud*/115_200);
        let brdh = brd >> 6;
        let brdf = brd % 64;
        uart0.cc.write(|w| w.cs().altclk());
        uart0.ibrd.write(|w| w.divint().bits(brdh as u16));
        uart0.fbrd.write(|w| w.divfrac().bits(brdf as u8));
        uart0.lcrh.write(|w| w.wlen()._8());
        uart0.ctl.modify(|_, w| w.uarten().bit(true));
    });
}

extern fn sys_tick(_: cortex_m::exception::SysTick) {
    cortex_m::interrupt::free(|cs| {
        let gpio_n = tm4c129x::GPIO_PORTN.borrow(cs);
        let uart0   = tm4c129x::UART0.borrow(cs);

        // Blink LED
        gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ 0x02));

        // Write to UART
        uart0.dr.write(|w| w.data().bits(b'A'));
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
