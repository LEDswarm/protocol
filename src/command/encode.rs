use std::convert::Into;

use super::{
    Command,
    Request,
    Response,
};

impl Into<Vec<u8>> for Command {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![];

        // Push our signature string so we can identify our own messages.
        for b in "ghoust".as_bytes().iter() {
            bytes.push(*b);
        }

        match self {
            Command::Request(request) => {
                bytes.push(1u8);
            },

            Command::Response(response) => {
                bytes.push(2u8);
            },
        }

        bytes
    }
}