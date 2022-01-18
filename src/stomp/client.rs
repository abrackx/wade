use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ClientCommand {
    SEND,
    SUBSCRIBE,
    UNSUBSCRIBE,
    BEGIN,
    COMMIT,
    ABORT,
    ACK,
    NACK,
    DISCONNECT,
    CONNECT,
    STOMP,
}

impl std::str::FromStr for ClientCommand {
    type Err = String;

    fn from_str(maybe_command: &str) -> Result<Self, Self::Err> {
        match maybe_command {
            "SEND" => Ok(ClientCommand::SEND),
            "SUBSCRIBE" => Ok(ClientCommand::SUBSCRIBE),
            "UNSUBSCRIBE" => Ok(ClientCommand::UNSUBSCRIBE),
            "BEGIN" => Ok(ClientCommand::BEGIN),
            "ABORT" => Ok(ClientCommand::ABORT),
            "ACK" => Ok(ClientCommand::ACK),
            "NACK" => Ok(ClientCommand::NACK),
            "DISCONNECT" => Ok(ClientCommand::DISCONNECT),
            "CONNECT" => Ok(ClientCommand::CONNECT),
            "STOMP" => Ok(ClientCommand::STOMP),
            _ => Err(format!("'{}' is not a valid value for ClientCommand", maybe_command))
        }
    }
}

impl fmt::Display for ClientCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
