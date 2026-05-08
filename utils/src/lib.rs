pub struct StdDelay;

impl embedded_hal::delay::DelayNs for StdDelay {
    fn delay_ns(&mut self, ns: u32) {
        std::thread::sleep(std::time::Duration::from_nanos(ns as u64));
    }
}

impl embedded_hal_async::delay::DelayNs for StdDelay {
    async fn delay_ns(&mut self, ns: u32) {
        tokio::time::sleep(std::time::Duration::from_nanos(ns as u64)).await;
    }
}
