use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;

#[derive(Deserialize)]
struct Request {
    path: String,
    method: Option<String>,
    headers: Option<Vec<(String, String)>>,
    body: Option<Vec<u8>>,
}

fn handler(r: Request, _: Context) -> Result<Vec<u8>, HandlerError> {
    let mut req = ureq::request(r.method.as_deref().unwrap_or("GET"), &r.path);
    if let Some(headers) = &r.headers {
        for (h, v) in headers {
            req.set(h, v);
        }
    }
    let resp = match r.body.as_deref() {
        Some(body) => req.send_bytes(body),
        None => req.call(),
    };
    let mut buf = match resp.header("Content-Length").and_then(|l| l.parse().ok()) {
        Some(len) => Vec::with_capacity(len),
        None => Vec::new(),
    };
    match resp.into_reader().read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(HandlerError::from(e.to_string().as_ref())),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda_runtime::lambda!(handler);
    Ok(())
}
