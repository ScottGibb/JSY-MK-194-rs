# Examples

The driver is built in such a way that it should be able to run on multiple platforms due to [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) and [`embedded-io`](https://docs.rs/embedded-io/latest/embedded_io/) as well as run in both sync and [async](https://doc.rust-lang.org/book/ch17-00-async-await.html) modes.

## Std Sync/Async Examples

The top level examples are designed to work with an operating system in a `std` environment

- [getters.rs](./getters.rs)
- [setters.rs](./setters.rs)
- [registers.rs](./registers.rs) This example uses the `advanced` feature which exposes more fine grained control allowing the user to access the individual registers.
- [power_loop_example.rs](./power_loop_example.rs)

- [tokio.rs](./tokio.rs) This examples uses the Tokio runtime to access the device asynchronously.

As for required Hardware you will need the following

- A OS Based system (MacOS/Linux)
- A [JSY-MK-194G](aliexpress.com/item/1005007369940517.html?_randl_currency=GBP&_randl_shipto=UK&src=google&src=google&albch=shopping&acnt=231-612-1468&isdl=y&slnk=&plac=&mtctp=&albbt=Google_7_shopping&aff_platform=google&aff_short_key=_oFgTQeV&gclsrc=aw.ds&&albagn=888888&&ds_e_adid=&ds_e_matchtype=&ds_e_device=c&ds_e_network=x&ds_e_product_group_id=&ds_e_product_id=en1005007369940517&ds_e_product_merchant_id=107916913&ds_e_product_country=GB&ds_e_product_language=en&ds_e_product_channel=online&ds_e_product_store_id=&ds_url_v=2&albcp=22967630628&albag=&isSmbAutoCall=false&needSmbHouyi=false&gad_source=1&gad_campaignid=22967633691&gbraid=0AAAABBR8kP2W36NR9oZrI7ziAkLI3uwNc&gclid=Cj0KCQjw2YDQBhD_ARIsAE1qeSc_cw1X4n2yD_FTpQ3Uamo0q90_YOL96oPukffMDUxBA4lZH9dQ3U4aAn6mEALw_wcB) Power Monitor
- A [USB-TTL Serial Converter](https://thepihut.com/products/ft232-usb-to-ttl-serial-cable?variant=27740896977&country=GB&currency=GBP&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&gad_source=1&gad_campaignid=22549809780&gbraid=0AAAAADfQ4GF7h5PbWXuqDde1qGW2aef6W&gclid=Cj0KCQjw2YDQBhD_ARIsAE1qeSce3H55MQ0PwCPOIDnVsoudHlD49f0gZG5OOSAOqsQPlIXi3mF7U7YaAinDEALw_wcB)

## No-std Examples

As fo `no-std` examples we have both an `async` and `sync` example both using the `defmt` feature for logging.

For the synchronous example, a Baremetal application using an [STM32 Blue Pill](https://www.amazon.co.uk/AZDelivery-STM32-Development-Module-Parent/dp/B07CRHX5F5?th=1) is demonstrated.

For the asynchronous example, an [Embassy application](https://embassy.dev/) using an [ESP32C3](https://thepihut.com/products/seeed-xiao-esp32c3) is used.
