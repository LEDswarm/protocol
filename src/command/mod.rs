mod encode;

pub enum Command {
    Request(Request),
    Response(Response),
}

pub enum Request {
    /// Initiate a connection to the host.
    Connect,
    /// Disconnect this controller if it is connected to the host.
    Disconnect,
    /// Received by a controller when the base station sets the color.
    SetColorRGBW(u8, u8, u8, u8),
    /// Sent by a controller to leave the current round since it has been moved too quickly.
    TooMuchAcceleration,
}

pub enum Response {
    /// The given request has succeeded.
    Ok,
    /// The given request has returned an error.
    Err,
    /// Sent by the host to signify a successful connection after a `Connect` request.
    Connected,
}