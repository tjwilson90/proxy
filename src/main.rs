use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;

#[derive(Deserialize)]
struct Request {
    path: String,
    method: Option<String>,
    #[serde(default)]
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

fn handler(r: Request, _: Context) -> Result<String, HandlerError> {
    let mut req = ureq::request(r.method.as_deref().unwrap_or("GET"), &r.path);
    for (h, v) in &r.headers {
        req.set(h, v);
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
        Ok(_) => Ok(base64::encode(buf)),
        Err(e) => Err(HandlerError::from(e.to_string().as_ref())),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda_runtime::lambda!(handler);
    Ok(())
}
