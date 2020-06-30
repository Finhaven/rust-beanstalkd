extern crate bufstream;

use std::net::TcpStream;
use self::bufstream::BufStream;
use std::io::{Write, BufRead, Read};
use std::str::FromStr;
use std::str::from_utf8;

use error::{BeanstalkdError, BeanstalkdResult};
use response::{Response, Status};

macro_rules! try {
    ($e:expr) => (match $e { Ok(e) => e, Err(_) => return Err(BeanstalkdError::RequestError) })
}

macro_rules! try_option {
    ($e:expr) => (match $e { Some(e) => e, None => return Err(BeanstalkdError::RequestError) })
}

pub struct Request<'a> {
    stream: &'a mut BufStream<TcpStream>,
}

impl<'a> Request<'a> {
    pub fn new<'b>(stream: &'b mut BufStream<TcpStream>) -> Request {
        Request { stream: stream }
    }

    pub fn send(&mut self, message: &[u8]) -> BeanstalkdResult<Response> {
        let _ = self.stream.write(message);
        let _ = self.stream.flush();

        let mut line = String::new();
        match self.stream.read_line(&mut line) {
            Ok(bytes_read) => {
                // Zero bytes read indicates the TCP connection was closed.
                if bytes_read == 0 {
                    return Err(BeanstalkdError::ConnectionError)
                }
            },
            Err(_) => return Err(BeanstalkdError::RequestError)
        };
        let line_segments: Vec<&str> = line.trim().split(' ').collect();
        let status_str = try_option!(line_segments.first());
        let status = match *status_str {
            "OK" => Status::OK,
            "RESERVED" => Status::RESERVED,
            "INSERTED" => Status::INSERTED,
            "USING" => Status::USING,
            "DELETED" => Status::DELETED,
            "WATCHING" => Status::WATCHING,
            "NOT_IGNORED" => Status::NOT_IGNORED,
            "NOT_FOUND" => Status::NOT_FOUND,
            "FOUND" => Status::FOUND,
            "TIMED_OUT" => Status::TIMED_OUT,
            "RELEASED" => Status::RELEASED,
            "BURIED" => Status::BURIED,
            "TOUCHED" => Status::TOUCHED,
            other => {
                dbg!(&status_str);
                return Err(BeanstalkdError::UnknownStatusError(other.to_string()));
            },
        };
        let mut data = line.clone();

        // These status codes indicate that there's a payload to decode
        let segment_offset_opt = match status {
            Status::OK => Some(1),
            Status::RESERVED | Status::FOUND => Some(2),

            _ => None,
        };

        if let Some(segment_offset) = segment_offset_opt {
            let bytes_count_str = try_option!(line_segments.get(segment_offset));
            let bytes_count: usize = try!(FromStr::from_str(*bytes_count_str));
            let mut tmp_vec: Vec<u8> = vec![0; bytes_count + 2]; // +2 needed for trailing line break
            let payload_utf8 = &mut tmp_vec[..];
            try!(self.stream.read_exact(payload_utf8));
            let payload_str = try!(from_utf8(&payload_utf8));
            data = data + &payload_str;
        }

        Ok(Response {
            status: status,
            data: data,
        })
    }
}
