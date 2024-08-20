use crate::Value;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KvError{
    #[error("not found for table:{0}, key:{1}")]
    NotFound(String, String),
    #[error("cannot parse command: {0}")]
    InvalidCommand(String),
    #[error("cannot convert {:0} to {1}")]
    ConvertError(Value, &'static str),
    #[error("cannot process command {0} with table: {1}, key: {2}, error: {}")]    
    StorageError(&'static str, String, String, String),
    #[error("failed to encode protobuf")]
    EncodeError(#[from] prost::EncodeError),
    #[error("failed to decode protobuf")]
    DecodeError(#[from] prost::DecodeError),
    #[error("internal error {0}")]
    Internal(String)

}