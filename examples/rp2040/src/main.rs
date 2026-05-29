//! # UART Example
//!
//! This application demonstrates how to use the UART Driver to talk to a serial
//! connection.
//!
//! It may need to be adapted to your particular board layout and/or pin
//! assignment.
//!
//! See the top-level `README.md` file for Copyright and license details.

#![no_std]
#![no_main]
use embedded_io::Write as _;
use embedded_io::{Read as _, ReadExactError}; // brings write_all into scope // brings read_exact into scope

use jsy_mk_194_rs::types::Baudrate;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_probe as _;

// defmt-rtt must be linked to enable RTT logging via probe-rs
use defmt_rtt as _;

// Alias for our HAL crate
use rp2040_hal::{self as hal};

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

use hal::fugit::RateExtU32;
use rp2040_hal::clocks::Clock;

// UART related types
use hal::uart::{DataBits, StopBits, UartConfig};

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then writes to the UART in
/// an infinite loop.
#[rp2040_hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();
    let _timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let uart_pins = (
        // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
        pins.gpio0.into_function(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pins.gpio1.into_function(),
    );
    defmt::info!("Configuring UART...");
    let baudrate = Baudrate::default();
    let baudrate = u32::from(baudrate);
    defmt::info!("Setting baudrate to {} bps", baudrate);

    let mut uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(baudrate.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    defmt::info!("RP2040 Example");
    defmt::info!("UART configured with baudrate: {}", Baudrate::default());
    defmt::info!("Waiting for JSY-MK-194G startup...");
    delay.delay_ms(1000);

    // defmt::info!("Setting up JSY-MK-194G driver");
    // let mut device = JsyMk194g::new(
    //     uart,
    //     Id::default(),
    //     timer,
    //     DEFAULT_REQUEST_RESPONSE_DELAY,
    //     DEFAULT_CHANNEL_REQUEST_RESPONSE_DELAY,
    // );

    loop {
        let write_request = [0x01, 0x03, 0x00, 0x48, 0x00, 0x0E, 0x44, 0x18];
        if let Err(err) = uart.write_all(&write_request) {
            defmt::warn!("write_all failed: {:?}", err);
            delay.delay_ms(100);
            continue;
        }

        // Read promptly to avoid UART FIFO overrun on RP2040.
        let mut read_response = [0u8; 61];
        if let Err(err) = uart.read_exact(&mut read_response[0..3]) {
            match err {
                ReadExactError::UnexpectedEof => {
                    defmt::warn!("Header read hit EOF")
                }
                ReadExactError::Other(io_err) => {
                    defmt::warn!("Header read I/O error: {:?}", io_err)
                }
            }
            delay.delay_ms(100);
            continue;
        }

        let byte_count = read_response[2] as usize;
        defmt::info!(
            "Received response header: id={} function={} byte_count={}",
            read_response[0],
            read_response[1],
            read_response[2]
        );

        // Read remaining bytes: payload + CRC16.
        let remaining_len = byte_count + 2;
        if remaining_len > (read_response.len() - 3) {
            defmt::warn!("Invalid byte_count {}", byte_count);
            delay.delay_ms(100);
            continue;
        }

        let response = uart.read_exact(&mut read_response[3..3 + remaining_len]);
        match response {
            Ok(()) => defmt::info!(
                "Received response payload+crc: {:?}",
                &read_response[3..3 + remaining_len]
            ),
            Err(e) => match e {
                ReadExactError::UnexpectedEof => defmt::warn!(
                    "Response payload was shorter than expected (expected {} bytes)",
                    remaining_len
                ),
                ReadExactError::Other(err) => {
                    defmt::warn!("I/O error while reading response payload: {:?}", err)
                }
            },
        }
        // Create a Read for all Channels request

        // defmt::info!("Scanning Channels...");
        // match device.get_all_channels() {
        //     Ok(channels) => defmt::info!("Channels: {:?}", channels),
        //     Err(err) => defmt::warn!("Channel read failed: {:?}", err),
        // }
        delay.delay_ms(200);
    }
}

// End of file
