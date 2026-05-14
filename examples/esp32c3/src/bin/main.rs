#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, with_timeout};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::uart::{Config, Uart};
use jsy_mk_194_rs::jsy_mk_194g::JsyMk194g;
use jsy_mk_194_rs::types::Baudrate;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("PANIC: {}", defmt::Display2Format(info));
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.2.0

    rtt_target::rtt_init_defmt!();
    info!("RTT initialized, starting ESP32C3 JSY example...");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    info!("Peripherals initialized");

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
    info!("Embassy RTOS started");

    // Use UART1 for meter communication (UART0 is typically console/bootloader)
    // ESP32C3 default UART1 pins: GPIO9=TX, GPIO10=RX
    let baud = u32::from(Baudrate::default());
    info!("Configuring UART1 at {} baud", baud);

    let uart_config = Config::default().with_baudrate(baud);
    let uart = Uart::new(peripherals.UART1, uart_config)
        .expect("UART1 init failed")
        .with_tx(peripherals.GPIO9)
        .with_rx(peripherals.GPIO10)
        .into_async();
    info!("UART1 configured on GPIO9(TX)/GPIO10(RX) at {} baud", baud);

    let delay = embassy_time::Delay;

    info!("Initializing JSY driver (will attempt to read device ID)...");
    let mut driver =
        match with_timeout(Duration::from_secs(3), JsyMk194g::new_default(uart, delay)).await {
            Ok(Ok(d)) => {
                info!("JSY driver initialized successfully!");
                d
            }
            Ok(Err(e)) => {
                error!("JSY driver init failed: {:?}", e);
                error!("Check wiring: ESP32 GPIO9→Meter RX, GPIO10→Meter TX, GND connected");
                panic!("JSY driver initialization failed");
            }
            Err(_) => {
                error!("JSY driver init timed out after 3 seconds");
                error!("Check: 1) Meter powered 2) Wiring correct 3) Baud rate matches");
                panic!("JSY driver initialization timed out");
            }
        };

    info!("Embassy initialized!");

    let _ = spawner;

    loop {
        info!("Reading async measurements...");
        Timer::after(Duration::from_secs(1)).await;

        match with_timeout(Duration::from_secs(2), driver.read_statistics()).await {
            Ok(Ok(stats)) => info!("Stats: {}", stats),
            Ok(Err(e)) => error!("Read failed: {:?}", e),
            Err(_) => error!("Read timed out after 2 seconds - meter not responding"),
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
