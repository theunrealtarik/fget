use reqwest::Url;
use serde::Deserialize;
use std::io::{Error, ErrorKind};
use tiny_http::{Response, Server};

pub static PORT: u64 = 57005;

#[derive(Deserialize, Default)]
pub struct Body {
    pub url: String,
    pub final_url: String,
}

pub fn listen() -> Result<(Url, Url), Error> {
    let address = format!("127.0.0.1:{}", PORT);
    let server =
        Server::http(address).map_err(|_| Error::new(ErrorKind::AddrInUse, "Address is in use"))?;

    for mut request in server.incoming_requests() {
        let mut buffer = String::new();
        request
            .as_reader()
            .read_to_string(&mut buffer)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to read request body"))?;

        let should_continue = request.url() == "/check";

        request
            .respond(Response::from_string(String::new()))
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to send response"))?;

        if should_continue {
            continue;
        }

        let parsed: Body = serde_json::from_str(&buffer)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse JSON body"))?;

        let url_parsed = Url::parse(&parsed.url);
        let final_url_parsed = Url::parse(&parsed.final_url);

        if url_parsed.is_ok() && final_url_parsed.is_ok() {
            return Ok((Result::unwrap(url_parsed), Result::unwrap(final_url_parsed)));
        } else {
            continue;
        }
    }

    Err(Error::new(
        ErrorKind::UnexpectedEof,
        "No incoming request received",
    ))
}
