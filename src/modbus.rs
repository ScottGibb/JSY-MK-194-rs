mod buffer_communications;
mod constants;
mod multi_register_communications;
mod protocol;
mod register_communications;
mod requests;
mod responses;
pub mod types;

pub use protocol::REQUEST_RESPONSE_DELAY;
pub use responses::ModbusErrorResponse;
pub use types::ErrorCode;
