use crate::request::{HttpHeader, HttpHeaderMap};
use crate::response::HttpResponse;

pub trait HttpResponseFormatter {
    fn format(&self, response: HttpResponse) -> HttpResponse;
}

pub struct DefaultHttpResponseFormatter {}

impl DefaultHttpResponseFormatter {
    fn insert_header_if_not_present(&self, key: HttpHeader, value: &str, headers: &mut HttpHeaderMap) {
        if !headers.contains_key(&key) {
            headers.insert(key, value.to_string());
        }
    }
}

impl HttpResponseFormatter for DefaultHttpResponseFormatter {
    fn format(&self, response: HttpResponse) -> HttpResponse {
        let mut headers = response.headers.clone();

        self.insert_header_if_not_present(HttpHeader::Date, "Tue, 03 Dec 2024 23:31:56 GMT", &mut headers);
        self.insert_header_if_not_present(HttpHeader::Server, "altaria", &mut headers);
        self.insert_header_if_not_present(HttpHeader::ContentType, "text/plain", &mut headers);
        self.insert_header_if_not_present(HttpHeader::ContentLength, &response.body.len().to_string(), &mut headers);
        self.insert_header_if_not_present(HttpHeader::Location, "localhost:8080", &mut headers);

        HttpResponse {
            status_code: response.status_code,
            headers,
            body: response.body
        }
    }
}