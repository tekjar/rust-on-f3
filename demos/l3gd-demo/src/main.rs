//! A LED roulette!
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate f3;
#[macro_use(iprint, iprintln)]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

// use cast::{usize, u8};
// use cortex_m::peripheral::SystClkSource;
// use f3::led::{self, LEDS};
use f3::prelude::*;
use f3::spi::Spi;
use rtfm::{app, Threshold};

// TASKS & RESOURCES
app! {
    device: f3::stm32f30x,

    tasks: {
        SPI1: {
            path: roulette,
            resources: [SPI1, ITM],
        },
    },
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals) {
    let spi = Spi(p.SPI1);
    spi.init(p.GPIOA, p.GPIOE, p.RCC);
    spi.enable();
    iprintln!(&p.ITM.stim[0], "Itm Init");
    spi.send(0x20, 0b1000_1111);

    if let Ok(byte) = spi.read() {
        iprintln!(&p.ITM.stim[0], "byte = {}", byte);
    }
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
fn roulette(_t: &mut Threshold, r: SPI1::Resources) {
    let spi = Spi(&**r.SPI1);
    iprintln!(&r.ITM.stim[0], "World");

    if let Ok(byte) = spi.read() {
        iprintln!(&r.ITM.stim[0], "byte = {}", byte);
    }
}
