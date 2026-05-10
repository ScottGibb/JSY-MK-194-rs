# Examples

The driver is designed to run on multiple platforms using [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) and [`embedded-io`](https://docs.rs/embedded-io/latest/embedded_io/), and to support both sync and [async](https://doc.rust-lang.org/book/ch17-00-async-await.html) modes.

## Std Sync/Async Examples

The top-level examples are designed to work with an operating system in a `std` environment.

- [getters.rs](./getters.rs)
- [setters.rs](./setters.rs)
- [registers.rs](./registers.rs) This example uses the `advanced` feature, which exposes more fine-grained control and allows users to access individual registers.
- [power_loop_example.rs](./power_loop_example.rs)

- [tokio.rs](./tokio.rs) This example uses the Tokio runtime to access the device asynchronously.

For the required hardware, you will need the following:

- An OS-based system (macOS/Linux)
- A [JSY-MK-194G](https://www.aliexpress.com/item/1005007369940517.html) power monitor
- A [USB-TTL Serial Converter](https://thepihut.com/products/ft232-usb-to-ttl-serial-cable?variant=27740896977&country=GB&currency=GBP&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&gad_source=1&gad_campaignid=22549809780&gbraid=0AAAAADfQ4GF7h5PbWXuqDde1qGW2aef6W&gclid=Cj0KCQjw2YDQBhD_ARIsAE1qeSce3H55MQ0PwCPOIDnVsoudHlD49f0gZG5OOSAOqsQPlIXi3mF7U7YaAinDEALw_wcB)

## No-std Examples

For `no-std` examples, we have both an `async` and `sync` example using the `defmt` feature for logging.

For the synchronous example, a bare-metal application using an [STM32 Blue Pill](https://www.amazon.co.uk/AZDelivery-STM32-Development-Module-Parent/dp/B07CRHX5F5?th=1) is demonstrated.

For the asynchronous example, an [Embassy application](https://embassy.dev/) using an [ESP32C3](https://thepihut.com/products/seeed-xiao-esp32c3) is used.
