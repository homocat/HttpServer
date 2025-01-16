use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct HttpRequest<B> {
    headers: HttpRequestHeader,
    body: Option<B>,
}

#[derive(Debug)]
pub struct HttpHeaders(HashMap<String, String>);

impl From<HashMap<String, String>> for HttpHeaders {
    fn from(value: HashMap<String, String>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct HttpRequestHeader {
    pub(crate) method: HttpMethod,
    pub(crate) url: String,
    pub(crate) version: HttpVersion,
    pub(crate) headers: HttpHeaders,
    pub(crate) host: String,
}

// impl< B> HttpRequest<B>
// where
//     B: ToString + Debug,
// {
//     // 创建一个新的 HTTP 请求
//     pub fn new(method: HttpMethod, url: String) -> Self {
//         HttpRequest {
//             headers: HashMap::new(),
//             body: None,
//         }
//     }

//     // 添加请求头
//     pub fn add_header(&mut self, key: &'r str, value: &'r str) {
//         self.headers.insert(key, value);
//     }

//     // 设置请求体
//     pub fn set_body(&mut self, body: B) {
//         self.body = Some(body);
//     }

//     // 格式化请求为 HTTP 1.1 请求字符串
//     pub fn to_string(&self) -> String {
//         let mut request = format!("{} {} HTTP/1.1\r\n", self.method.to_string(), self.url);

//         // 添加请求头
//         for (key, value) in &self.headers {
//             request.push_str(&format!("{}: {}\r\n", key, value));
//         }

//         // 如果有请求体，添加一个空行后再附上请求体
//         if let Some(body) = &self.body {
//             request.push_str("\r\n"); // 空行分隔头部和体
//             request.push_str(&format!("{:?}", body));
//         }

//         request.push_str("\r\n"); // 请求末尾的 CRLF
//         request
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
            HttpMethod::PATCH => "PATCH".to_string(),
            HttpMethod::HEAD => "HEAD".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HttpVersion {
    HTTP_1_0,
    HTTP_1_1,
    HTTP_2_0,
}

impl ToString for HttpVersion {
    fn to_string(&self) -> String {
        match self {
            HttpVersion::HTTP_1_0 => "HTTP/1.0".to_string(),
            HttpVersion::HTTP_1_1 => "HTTP/1.1".to_string(),
            HttpVersion::HTTP_2_0 => "HTTP/2.0".to_string(),
        }
    }
}
