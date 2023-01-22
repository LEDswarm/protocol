use super::{
    Command,
    Request,
    Response,
};

use crate::types::{
    ProtocolResult,
    ProtocolError,
};

/// Try to assemble a `Command` instance from a sequence of bytes.
///
/// This function first checks if the sequence is long enough, and if the first six bytes are
/// the signature ASCII string "ghoust". It then proceeds to read the actual commands using a
/// simple bytecode-like encoding of the individual commands.
pub fn decode(bytes: Vec<u8>) -> ProtocolResult<Command> {
    // Check that our message has the required length.
    // Ends will be padded with zeroes if the message happens to be shorter.
    if bytes.len() != 12 {
        return Err(ProtocolError::UnexpectedLength);
    }

    // Check for the signature "ghoust" ASCII string.
    if std::str::from_utf8(&bytes[0 ..= 5]) != Ok("ghoust") {
        return Err(ProtocolError::NoSignatureFound);
    }

    match &bytes[6] {
        // Request
        1 => {
            match &bytes[7] {
                1 => Ok(Command::Request(Request::Connect)),
                2 => Ok(Command::Request(Request::Disconnect)),
                3 => Ok(Command::Request(Request::TooMuchAcceleration)),
                4 => Ok(Command::Request(Request::SetColorRGBW(
                    bytes[8],
                    bytes[9],
                    bytes[10],
                    bytes[11],
                ))),

                _ => Err(ProtocolError::InvalidCommandByte),
            }
        },

        // Response
        2 => {
            match &bytes[7] {
                1 => Ok(Command::Response(Response::Ok)),
                2 => Ok(Command::Response(Response::Err)),
                3 => Ok(Command::Response(Response::Connected)),

                _ => Err(ProtocolError::InvalidCommandByte),
            }
        },

        _ => Err(ProtocolError::InvalidCommandByte),
    }
}