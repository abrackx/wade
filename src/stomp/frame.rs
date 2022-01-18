use bytes::{BufMut, BytesMut};

pub struct StompFrame<T: std::fmt::Display> {
    pub command: T,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl<T: std::fmt::Display> StompFrame<T> {
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

    fn deserialize(_frame: Vec<u8>) -> StompFrame<T> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::stomp::client::ClientCommand;
    use crate::stomp::server::ServerCommand;

    use super::*;

    #[test]
    fn should_parse() {
        let data = b"CONNECT
accept-version:1.2
host:example.com
login:user
passcode:password\n\n\x00"
            .to_vec();
        let frame: StompFrame<ClientCommand> = StompFrame::deserialize(data);
        assert_eq!(frame.command, ClientCommand::CONNECT);
    }

    #[test]
    fn should_write_client() {
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

    #[test]
    fn should_write_server() {
        let data = b"MESSAGE
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
            command: ServerCommand::MESSAGE,
            headers,
            body: None,
        };
        assert_eq!(data, frame.serialize());
    }
}