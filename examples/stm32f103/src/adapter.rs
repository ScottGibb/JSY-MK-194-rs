// TODO: remove this once, stm32f1 bumps to latested embedded-io and we can remove this adapter
use embedded_hal_nb::serial::Write as NbWrite;
use embedded_io::{ErrorKind, ErrorType, Read, Write};
use stm32f1xx_hal::pac::USART3;
use stm32f1xx_hal::serial::{Rx, Serial, Tx};

/// Adapter wrapping stm32f1xx_hal's Serial type to implement embedded_io 0.7 traits.
/// Needed because stm32f1xx-hal implements embedded-io 0.6, but this library requires 0.7.
pub struct SerialAdapter {
    tx: Tx<USART3>,
    rx: Rx<USART3>,
}

impl SerialAdapter {
    /// Create a new SerialAdapter from a stm32f1xx_hal Serial instance
    pub fn new<Otype, PULL>(serial: Serial<USART3, Otype, PULL>) -> Self {
        let (tx, rx) = serial.split();
        SerialAdapter { tx, rx }
    }
}

impl ErrorType for SerialAdapter {
    type Error = ErrorKind;
}

impl Read for SerialAdapter {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ErrorKind> {
        for (i, byte) in buf.iter_mut().enumerate() {
            match nb::block!(self.rx.read()) {
                Ok(b) => *byte = b,
                Err(_) => {
                    return if i > 0 { Ok(i) } else { Err(ErrorKind::Other) };
                }
            }
        }
        Ok(buf.len())
    }
}

impl Write for SerialAdapter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, ErrorKind> {
        for (i, &byte) in buf.iter().enumerate() {
            // Use UFCS to avoid resolving to embedded_io 0.6's Write (which Tx also implements)
            match nb::block!(<Tx<USART3> as NbWrite<u8>>::write(&mut self.tx, byte)) {
                Ok(_) => {}
                Err(_) => return Ok(i),
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), ErrorKind> {
        Ok(())
    }
}
