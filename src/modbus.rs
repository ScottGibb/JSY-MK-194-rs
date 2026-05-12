mod buffer_communications;
mod constants;
mod multi_register_communications;
mod protocol;
mod register_communications;
mod requests;
mod responses;
mod types;

pub use protocol::DEFAULT_CHANNEL_REQUEST_RESPONSE_DELAY;
pub use protocol::DEFAULT_REQUEST_RESPONSE_DELAY;
pub use responses::ModbusErrorResponse;
pub use types::ErrorCode;
pub use types::FunctionCode;
