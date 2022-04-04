use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub pathname: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub struct HttpResponse<'a> {
    pub status_code: i16,
    pub status_message: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<&'a [u8]>,
}

