pub type ProtocolResult<T> = Result<T, ProtocolError>;

pub enum ProtocolError {
    UnexpectedEOF,
}