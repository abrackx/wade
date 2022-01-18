use std::fmt;

use bytes::{BufMut, BytesMut};

pub struct StompFrame {
    pub command: ClientCommand,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

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


impl StompFrame {
    pub fn serialize(&self) -> Vec<u8> {
        fn write_escaped(byte: u8, buffer: &mut BytesMut) {
            match byte {
                b'\r' => {
                    buffer.put_u8(b'\\');
                    buffer.put_u8(b'r')
                }
                b'\n' => {
                    buffer.put_u8(b'\\');
                    buffer.put_u8(b'n')
                }
                b':' => {
                    buffer.put_u8(b'\\');
                    buffer.put_u8(b'c')
                }
                b'\\' => {
                    buffer.put_u8(b'\\');
                    buffer.put_u8(b'\\')
                }
                byte => buffer.put_u8(byte),
            }
        }
        let mut buffer = BytesMut::new();
        buffer.put_slice(self.command.to_string().as_bytes());
        buffer.put_u8(b'\n');
        self.headers.iter().for_each(|(key, val)| {
            for byte in key.as_bytes() {
                write_escaped(*byte, &mut buffer);
            }
            buffer.put_u8(b':');
            for byte in val.as_bytes().iter() {
                write_escaped(*byte, &mut buffer);
            }
            buffer.put_u8(b'\n');
        });
        if let Some(body) = self.body.as_ref() {
            //todo: add content length header
            buffer.put_u8(b'\n');
            buffer.put_slice(body.as_bytes());
        } else {
            buffer.put_u8(b'\n');
        }
        buffer.put_u8(b'\x00');
        buffer.to_vec()
    }

    fn deserialize(_frame: Vec<u8>) -> StompFrame {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let data = b"CONNECT
accept-version:1.2
host:example.com
login:user
passcode:password\n\n\x00"
            .to_vec();
        let frame = StompFrame::deserialize(data);
        assert_eq!(frame.command, ClientCommand::CONNECT);
    }

    #[test]
    fn should_write() {
        let data = b"CONNECT
foo:foo
accept-version:1.2
host:example.com\n\n\x00"
            .to_vec();
        let headers = vec![
            ("foo".to_string(), "foo".to_string()),
            ("accept-version".to_string(), "1.2".to_string()),
            ("host".to_string(), "example.com".to_string()),
        ];
        let frame = StompFrame {
            command: ClientCommand::CONNECT,
            headers,
            body: None,
        };
        assert_eq!(data, frame.serialize());
    }
}