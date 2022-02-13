use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Command {
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
    CONNECTED,
    MESSAGE,
    RECEIPT,
    ERROR,
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(maybe_command: &str) -> Result<Self, Self::Err> {
        match maybe_command {
            "SEND" => Ok(Command::SEND),
            "SUBSCRIBE" => Ok(Command::SUBSCRIBE),
            "UNSUBSCRIBE" => Ok(Command::UNSUBSCRIBE),
            "BEGIN" => Ok(Command::BEGIN),
            "ABORT" => Ok(Command::ABORT),
            "ACK" => Ok(Command::ACK),
            "NACK" => Ok(Command::NACK),
            "DISCONNECT" => Ok(Command::DISCONNECT),
            "CONNECT" => Ok(Command::CONNECT),
            "STOMP" => Ok(Command::STOMP),
            "CONNECTED" => Ok(Command::CONNECTED),
            "MESSAGE" => Ok(Command::MESSAGE),
            "RECEIPT" => Ok(Command::RECEIPT),
            "ERROR" => Ok(Command::ERROR),
            _ => Err(format!("'{}' is not a valid value command!", maybe_command))
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
