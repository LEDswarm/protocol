pub type ProtocolResult<T> = Result<T, ProtocolError>;

pub enum ProtocolError {
    /// The message does not start with a "ghoust" ASCII byte string.
    NoSignatureFound,
    /// The message does not have the right format.
    UnexpectedLength,
    /// A particular binary command code was not recognized.
    InvalidCommandByte,
}