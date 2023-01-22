use super::{
    Command,
    Request,
    Response,
};

pub fn encode(command: Command) -> Vec<u8> {
    let mut bytes = vec![];

    // Push our signature string so we can be sure messages belong to us.
    for b in "ghoust".as_bytes().iter() {
        bytes.push(*b);
    }

    match command {
        Command::Request(request) => {
            bytes.push(1u8);

            match request {
                Request::Connect             => bytes.push(1u8),
                Request::Disconnect          => bytes.push(2u8),
                Request::TooMuchAcceleration => bytes.push(3u8),
                Request::SetColorRGBW(r, g, b, w) => {
                    bytes.push(4u8);
                    bytes.push(r);
                    bytes.push(g);
                    bytes.push(b);
                    bytes.push(w);
                },
            }
        },

        Command::Response(response) => {
            bytes.push(2u8);

            match response {
                Response::Ok        => bytes.push(1u8),
                Response::Err       => bytes.push(2u8),
                Response::Connected => bytes.push(3u8),
            }
        },
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_connect() {
        let encoded = encode(Command::Request(Request::Connect));

        assert_eq!(
            encoded,
            // The first six bytes contain the ASCII string "ghoust".
            vec![
                103, 104, 111, 117, 115, 116,
                1,
                1,
            ],
        );
    }
}