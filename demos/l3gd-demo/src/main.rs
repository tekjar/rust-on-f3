//! A LED roulette!
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate f3;
#[macro_use(iprint, iprintln)]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
// extern crate nb;

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
            path: gyro,
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

    const WRITE: u8 = 0 << 7;
    p.GPIOE.bsrr.write(|w| w.br3().set_bit());
    while spi.send(0x20 | WRITE).is_err() {}
    while spi.send(0b1000_1111).is_err() {}


    for _ in 0..2 {
        while spi.read().is_err() {}
    }
    p.GPIOE.bsrr.write(|w| w.bs3().set_bit());

    'read: loop {
        p.GPIOE.bsrr.write(|w| w.br3().set_bit());

        let bytes = [0; 6];
        const MS: u8 = 1 << 6;
        const READ: u8 = 1 << 7;
        const OUT_X_L: u8 = 0x28;

        while spi.send(READ | MS | OUT_X_L).is_err() {}
        while spi.read().is_err() {}

        for _ in bytes.iter() {
            while spi.send(0x00).is_err() {}

            'retry: loop {
                let read = spi.read();
                 match read {
                    Ok(val) => {
                        iprintln!(&p.ITM.stim[0], "byte = {}", val);
                        break 'retry
                    }
                    Err(_) => continue 'retry
                }
            }
        }

        p.GPIOE.bsrr.write(|w| w.bs3().set_bit());
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
fn gyro(_t: &mut Threshold, r: SPI1::Resources) {
    let spi = Spi(&**r.SPI1);
    iprintln!(&r.ITM.stim[0], "World");

    if let Ok(byte) = spi.read() {
        iprintln!(&r.ITM.stim[0], "byte = {}", byte);
    }
}