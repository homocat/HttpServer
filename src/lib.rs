mod error;
mod request;

use std::{
    collections::HashMap,
    io::{BufRead, Read},
    net::TcpStream,
};

use anyhow::{Error, Ok};
use error::RequestError;
use nom::bytes::complete::{tag, take_until};
use request::{HttpMethod, HttpRequest, HttpRequestHeader, HttpVersion};

const DELIMITER: &[u8] = b"\r\n\r\n";

pub fn read_http_request(stream: &mut TcpStream) -> anyhow::Result<HttpRequestHeader> {
    let mut buf = [0, 255];
    let mut header_buf = vec![];
    loop {
        let n = stream.read(&mut buf)?;
        header_buf.extend_from_slice(&buf[..n]);
        if buf[..n].windows(4).any(|w| w == DELIMITER) {
            break;
        }
    }

    parse_header(&buf)
}

fn parse_header(buf: &[u8]) -> anyhow::Result<HttpRequestHeader> {
    let (i, _method_bytes) = tag::<_, _, nom::error::Error<&[u8]>>(b"GET ")(buf)
        .map_err(|_| RequestError::ParseHeaderError)?;
    let method = HttpMethod::GET;
    let (i, url) = take_until::<_, _, nom::error::Error<&[u8]>>(" ")(buf)
        .map_err(|_| RequestError::ParseHeaderError)?;
    let (i, _) = tag::<_, _, nom::error::Error<&[u8]>>(b" ")(i)
        .map_err(|_| RequestError::ParseHeaderError)?;
    let url = String::from_utf8_lossy(url).to_string();
    let (i, version_bytes) = take_until::<_, _, nom::error::Error<&[u8]>>("\n")(buf)
        .map_err(|_| RequestError::ParseHeaderError)?;
    let version = match version_bytes {
        b"1.0" => HttpVersion::HTTP_1_0,
        b"1.1" => HttpVersion::HTTP_1_1,
        b"2" => HttpVersion::HTTP_2_0,
        _ => unimplemented!("version not support yet"),
    };
    let i = consume_newline(i)?;
    let (i, _) = tag::<_, _, nom::error::Error<&[u8]>>(b"Host: ")(i)
        .map_err(|_| RequestError::ParseHeaderError)?;
    let (i, host_bytes) = take_until::<_, _, nom::error::Error<&[u8]>>("Host: ")(i)
        .map_err(|_| RequestError::ParseHeaderError)?;
    let host = String::from_utf8_lossy(host_bytes).to_string();
    let i = consume_newline(i)?;

    let mut headers = HashMap::new();
    let header_lines = String::from_utf8_lossy(i);
    for l in header_lines.lines() {
        let mut splits = l.split(": ");
        let name = splits.next().ok_or(RequestError::ParseHeaderError)?;
        let value = splits.next().ok_or(RequestError::ParseHeaderError)?;
        let (i, _) = tag::<_, _, nom::error::Error<&[u8]>>(": ")(i)
            .map_err(|_| RequestError::ParseHeaderError)?;
        headers.insert(name.to_string(), value.to_string());
    }

    Ok(HttpRequestHeader {
        method,
        url,
        version,
        headers: headers.into(),
        host,
    })
}

fn consume_newline(i: &[u8]) -> anyhow::Result<&[u8]> {
    let (i, _) = tag::<_, _, nom::error::Error<&[u8]>>(b"\n")(i)
        .map_err(|_| RequestError::SkipNewlineError)?;
    Ok(i)
}
