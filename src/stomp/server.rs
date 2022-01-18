use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ServerCommand {
    CONNECTED,
    MESSAGE,
    RECEIPT,
    ERROR,
}

impl std::str::FromStr for ServerCommand {
    type Err = String;

    fn from_str(maybe_command: &str) -> Result<Self, Self::Err> {
        match maybe_command {
            "CONNECTED" => Ok(ServerCommand::CONNECTED),
            "MESSAGE" => Ok(ServerCommand::MESSAGE),
            "RECEIPT" => Ok(ServerCommand::RECEIPT),
            "ERROR" => Ok(ServerCommand::ERROR),
            _ => Err(format!("'{}' is not a valid value for ServerCommand", maybe_command))
        }
    }
}

impl fmt::Display for ServerCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}