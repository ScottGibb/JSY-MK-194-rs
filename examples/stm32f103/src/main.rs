//! Serial interface loopback test
//!
//! You have to short the TX and RX pins to make this program work

#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use defmt_rtt as _;
use jsy_mk_194_rs::{jsy_mk_194g::JsyMk194g, types::Baudrate};
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, serial::Config};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let syst = cortex_m::Peripherals::take().unwrap().SYST;

    // Prepare the alternate function I/O registers
    //let mut afio = p.AFIO.constrain(&mut rcc);

    // Prepare the GPIOB peripheral
    let mut gpiob = p.GPIOB.split(&mut rcc);

    // USART1
    // let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    // Configure pb10 as a push_pull output, this will be the tx pin
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // Take ownership over pb11
    let rx = gpiob.pb11;

    // Set up the usart device. Take ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.

    let baudrate = u32::from(u8::from(Baudrate::_4800));

    let serial = p.USART3.serial(
        (tx, rx),
        Config::default().baudrate(baudrate.bps()),
        &mut rcc,
    );

    let delay = syst.delay(&rcc.clocks);

    let mut device = JsyMk194g::new_default(serial, delay).expect("This should not fail");

    loop {
        let stats = device.get_all_channels().unwrap();
        defmt::info!("Stats: {:?}", stats);
        cortex_m::asm::delay(8_000_000);
    }
}
