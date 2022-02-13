use std::ascii::escape_default;
use crate::stomp::command::Command;

#[derive(PartialEq, Debug)]
pub struct Frame {
    pub command: Command,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub fn serialize(frame: Frame) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.extend_from_slice(frame.command.to_string().as_bytes());
    bytes.push(b'\n');
    frame.headers.iter().for_each(|(key, val)| {
        for byte in key.as_bytes() {
            bytes.extend_from_slice(escape_default(*byte).to_string().as_bytes());
        }
        bytes.push(b':');
        for byte in val.as_bytes() {
            bytes.extend_from_slice(escape_default(*byte).to_string().as_bytes());
        }
        bytes.push(b'\n');
    });
    bytes.push(b'\n');
    if let Some(body) = frame.body {
        bytes.extend_from_slice(body.as_bytes());
    }
    bytes.push(b'\x00');
    bytes
}

pub fn deserialize(maybe_frame: Vec<u8>) -> Result<Frame, ()> {
    match std::str::from_utf8(&maybe_frame) {
        Ok(valid_utf8_str) => {
            let frame_split: Vec<&str> = valid_utf8_str.split("\n").collect();
            let string_command = frame_split[0];
            let command = Command::from(string_command.parse().unwrap());
            let mut body = None;
            let mut headers: Vec<(String, String)> = vec![];
            for (i, x) in frame_split.iter().enumerate().skip(1) {
                if x.is_empty() {
                    let mut almost_body = frame_split[i + 1].to_string();
                    almost_body.pop();
                    body = Some(almost_body);
                    break;
                }
                let spl: Vec<&str> = x.split(":").collect();
                headers.push((spl[0].to_string(), spl[1].to_string()));
            }
            Ok(Frame {
                command,
                headers,
                body,
            })
        }
        Err(_) => { Err(()) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let data = b"MESSAGE
accept-version:1.2
host:example.com
login:user
passcode:password\n\nsome body\x00"
            .to_vec();
        let headers = vec![
            ("accept-version".to_string(), "1.2".to_string()),
            ("host".to_string(), "example.com".to_string()),
            ("login".to_string(), "user".to_string()),
            ("passcode".to_string(), "password".to_string()),
        ];
        let body = Some("some body".to_string());
        let expected = Frame {
            command: Command::MESSAGE,
            headers,
            body,
        };
        let frame: Frame = deserialize(data).expect("test");
        assert_eq!(expected, frame);
    }

    #[test]
    fn should_write_client() {
        let data = b"CONNECT
foo:foo
accept-version:1.2
content-length:2
host:example.com\n\n\x00"
            .to_vec();
        let headers = vec![
            ("foo".to_string(), "foo".to_string()),
            ("accept-version".to_string(), "1.2".to_string()),
            ("content-length".to_string(), "2".to_string()),
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