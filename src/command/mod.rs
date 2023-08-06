#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Request(Request),
    Response(Response),
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameMode {
    LastOneStanding,
    // Territory,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Request {
    /// Initiate a connection to the host.
    Connect,
    /// Disconnect a controller from either side if the connection is currently active.
    Disconnect,
    /// Start a new round from the server side.
    StartRound(GameMode),
    /// Sent by a controller to exit the current round if a threshold is passed.
    ExitRound,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Response {
    /// The given request has succeeded.
    Ok,
    /// The given request has returned an error.
    Err,
    /// Sent by the host to signify a successful connection after a `Connect` request.
    Connected {
        id: u8,
    },
}

#[cfg(test)]
mod tests {
    use crate::types::ProtocolResult;
    use super::*;

    fn encode_decode(cmd: Command) -> ProtocolResult<Command> {
        let encoded = encode(cmd);

        decode(encoded)
    }

    #[test]
    fn integration_req_connect() {
        let decoded = encode_decode(Command::Request(Request::Connect));

        assert_eq!(
            decoded,
            Ok(Command::Request(Request::Connect)),
        );
    }

    #[test]
    fn integration_req_disconnect() {
        let decoded = encode_decode(Command::Request(Request::Disconnect));

        assert_eq!(
            decoded,
            Ok(Command::Request(Request::Disconnect)),
        );
    }

    #[test]
    fn integration_req_too_much_acceleration() {
        let decoded = encode_decode(Command::Request(Request::TooMuchAcceleration));

        assert_eq!(
            decoded,
            Ok(Command::Request(Request::TooMuchAcceleration)),
        );
    }

    #[test]
    fn integration_req_set_color_rgbw() {
        let decoded = encode_decode(Command::Request(Request::SetColorRGBW(210, 62, 98, 20)));

        assert_eq!(
            decoded,
            Ok(Command::Request(Request::SetColorRGBW(210, 62, 98, 20))),
        );
    }

    #[test]
    fn integration_res_ok() {
        let decoded = encode_decode(Command::Response(Response::Ok));

        assert_eq!(
            decoded,
            Ok(Command::Response(Response::Ok)),
        );
    }

    #[test]
    fn integration_res_err() {
        let decoded = encode_decode(Command::Response(Response::Err));

        assert_eq!(
            decoded,
            Ok(Command::Response(Response::Err)),
        );
    }

    #[test]
    fn integration_res_connected() {
        let decoded = encode_decode(Command::Response(Response::Err));

        assert_eq!(
            decoded,
            Ok(Command::Response(Response::Err)),
        );
    }
}