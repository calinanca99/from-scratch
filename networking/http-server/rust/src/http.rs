#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
pub enum Method {
    GET,
}

impl TryFrom<&str> for Method {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::GET),
            _ => Err("method not allowed".to_string()),
        }
    }
}

pub struct Request {
    method: Method,
    route: String,
}

impl Request {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn route(&self) -> &str {
        self.route.as_str()
    }
}

impl TryFrom<&str> for Request {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let method = Method::try_from(
            value
                .split_whitespace()
                .next()
                .ok_or("bad request".to_string())?,
        )?;
        let route = value
            .split_whitespace()
            .nth(1)
            .ok_or("bad request".to_string())?;

        Ok(Self {
            method,
            route: route.to_string(),
        })
    }
}

pub enum Status {
    Ok,
    BadRequest,
    MethodNotAllowed,
    NotFound,
}

pub struct Response {
    status: Status,
    body: Option<String>,
}

impl Response {
    pub fn new(status: Status, body: Option<String>) -> Self {
        Self { status, body }
    }
}

impl From<Response> for String {
    fn from(response: Response) -> String {
        let status = match response.status {
            Status::Ok => "200 OK",
            Status::BadRequest => "400 Bad Request",
            Status::MethodNotAllowed => "405 Method Not Allowed",
            Status::NotFound => "404 Not Found",
        };

        if let Some(body) = response.body {
            format!(
                "HTTP/1.1 {}\r\nnServer: from-scratch\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}",
                status, body.as_bytes().len(), body
            )
        } else {
            format!("HTTP/1.1 {}\r\nnServer: from-scratch\r\n", status)
        }
    }
}
