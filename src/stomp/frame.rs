use crate::stomp::command::Command;

pub struct Frame {
    pub command: Command,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub fn serialize(frame: Frame) -> Vec<u8> {
    fn write_escaped(byte: u8, buffer: &mut Vec<u8>) {
        match byte {
            b'\r' => {
                buffer.push(b'\\');
                buffer.push(b'r')
            }
            b'\n' => {
                buffer.push(b'\\');
                buffer.push(b'n')
            }
            b':' => {
                buffer.push(b'\\');
                buffer.push(b'c')
            }
            b'\\' => {
                buffer.push(b'\\');
                buffer.push(b'\\')
            }
            byte => buffer.push(byte),
        }
    }
    let mut buffer = vec![];
    buffer.extend_from_slice(frame.command.to_string().as_bytes());
    buffer.push(b'\n');
    frame.headers.iter().for_each(|(key, val)| {
        for byte in key.as_bytes() {
            write_escaped(*byte, &mut buffer);
        }
        buffer.push(b':');
        for byte in val.as_bytes().iter() {
            write_escaped(*byte, &mut buffer);
        }
        buffer.push(b'\n');
    });
    if let Some(body) = frame.body {
        //todo: add content length header
        buffer.push(b'\n');
        buffer.extend_from_slice(body.as_bytes());
    } else {
        buffer.push(b'\n');
    }
    buffer.push(b'\x00');
    buffer
}

pub fn deserialize(frame: Vec<u8>) -> Frame {
    let maybe_frame = match std::str::from_utf8(&frame) {
        Ok(frame) => {
            println!("{}", frame);
        }
        Err(_) => {}
    };

    Frame {
        command: Command::ACK,
        headers: vec![],
        body: Some("Test".to_string()),
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
        let frame: Frame = deserialize(data);
        assert_eq!(frame.command, Command::CONNECT);
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
        let frame = Frame {
            command: Command::CONNECT,
            headers,
            body: None,
        };
        assert_eq!(data, serialize(frame));
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
        let frame = Frame {
            command: Command::MESSAGE,
            headers,
            body: None,
        };
        assert_eq!(data, serialize(frame));
    }
}