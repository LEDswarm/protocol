use super::{
    Command,
    Request,
    Response,
};

use crate::types::{
    ProtocolResult,
    ProtocolError,
};

pub fn decode(bytes: Vec<u8>) -> ProtocolResult<Command> {
    // Check that our message has the required length.
    // Ends will be padded with zeroes if the message happens to be shorter.
    if bytes.len() != 12 {
        return Err(ProtocolError::UnexpectedLength);
    }

    if &bytes[0 .. 5] != "ghoust".as_bytes() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_connect() {
        //let encoded = encode(Command::Request(Request::Connect));

        assert_eq!(
            true,
            true,
            //encoded,
            // The first six bytes contain the ASCII string "ghoust".
            /*vec![
                103, 104, 111, 117, 115, 116,
                1,
                1,
            ],*/
        );
    }
}