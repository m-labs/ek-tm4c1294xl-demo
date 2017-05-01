#![feature(used, const_fn)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate tm4c129x;

use core::cell::Cell;
use cortex_m::ctxt::Local;
use cortex_m::exception::Handlers as ExceptionHandlers;
use tm4c129x::interrupt::Handlers as InterruptHandlers;

fn main() {
    hprintln!("Hello, world!");

    cortex_m::interrupt::free(|cs| {
        let systick = tm4c129x::SYST.borrow(cs);
        let sysctl  = tm4c129x::SYSCTL.borrow(cs);
        let gpio_a  = tm4c129x::GPIO_PORTA_AHB.borrow(cs);
        let gpio_f  = tm4c129x::GPIO_PORTF_AHB.borrow(cs);
        let gpio_n  = tm4c129x::GPIO_PORTN.borrow(cs);
        let uart0   = tm4c129x::UART0.borrow(cs);
        let pwm0    = tm4c129x::PWM0.borrow(cs);

        // Set up system timer
        systick.set_reload(systick.get_ticks_per_10ms());
        systick.enable_counter();
        systick.enable_interrupt();

        // Set up LED
        sysctl.rcgcgpio.modify(|_, w| w.r12().bit(true));
        while !sysctl.prgpio.read().r12().bit() {}

        gpio_n.dir.write(|w| w.dir().bits(0x02));
        gpio_n.den.write(|w| w.den().bits(0x02));

        // Set up UART0
        sysctl.rcgcgpio.modify(|_, w| w.r0().bit(true));
        while !sysctl.prgpio.read().r0().bit() {}

        gpio_a.dir.write(|w| w.dir().bits(0x02));
        gpio_a.den.write(|w| w.den().bits(0x03));
        gpio_a.afsel.write(|w| w.afsel().bits(0x03));
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

        // Set up PWM0
        sysctl.rcgcgpio.modify(|_, w| w.r5().bit(true));
        while !sysctl.prgpio.read().r5().bit() {}

        gpio_f.dir.write(|w| w.dir().bits(0x01));
        gpio_f.den.write(|w| w.den().bits(0x01));
        gpio_f.afsel.write(|w| w.afsel().bits(0x01));
        gpio_f.pctl.write(|w| unsafe { w.pmc0().bits(6) });

        sysctl.rcgcpwm.modify(|_, w| w.r0().bit(true));
        while !sysctl.prpwm.read().r0().bit() {}

        let load = (/*pwmclk*/16_000_000u32 / /*freq*/100_000) as u16;
        pwm0._0_gena.write(|w| w.actload().zero().actcmpad().one());
        pwm0._0_load.write(|w| w.load().bits(load));
        pwm0._0_cmpa.write(|w| w.compa().bits(0));
        pwm0._0_ctl.write(|w| w.enable().bit(true));
        pwm0.enable.write(|w| w.pwm0en().bit(true));
    });
}

use cortex_m::exception::SysTick;

extern fn sys_tick(ctxt: SysTick) {
    static ELAPSED: Local<Cell<u32>, SysTick> = Local::new(Cell::new(0));
    let elapsed = ELAPSED.borrow(&ctxt);

    elapsed.set(elapsed.get() + 1);

    cortex_m::interrupt::free(|cs| {
        let gpio_n = tm4c129x::GPIO_PORTN.borrow(cs);
        let uart0  = tm4c129x::UART0.borrow(cs);
        let pwm0   = tm4c129x::PWM0.borrow(cs);

        // Every 1 s...
        if elapsed.get() % 100 == 0 {
            // Blink LED
            gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ 0x02));

            // Write to UART0
            uart0.dr.write(|w| w.data().bits(b'A'));
        }

        // Every 10 ms...
        {
            // Change PWM0 duty cycle
            pwm0._0_cmpa.modify(|r, w| {
                let thresh = r.compa().bits();
                let thresh = (thresh + 1) % 100;
                w.compa().bits(thresh)
            });
        }
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
