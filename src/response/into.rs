use crate::headers;
use crate::response::{HttpResponse, HttpStatusCode};

pub trait IntoResponse {
    fn into_response(self) -> HttpResponse;
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> HttpResponse {
        self
    }
}

impl IntoResponse for &str {
    fn into_response(self) -> HttpResponse {
        HttpResponse {
            status_code: HttpStatusCode::OK,
            headers: headers! {
                ContentType: "text/plain"
            },
            body: self.as_bytes().to_vec()
        }
    }
}

impl IntoResponse for String {
    fn into_response(self) -> HttpResponse {
        HttpResponse {
            status_code: HttpStatusCode::OK,
            headers: headers! {
                ContentType: "text/plain"
            },
            body: self.into_bytes()
        }
    }
}

impl IntoResponse for HttpStatusCode {
    fn into_response(self) -> HttpResponse {
        HttpResponse {
            status_code: self,
            headers: headers! {
                ContentType: "text/plain"
            },
            body: vec![]
        }
    }
}

impl<T> IntoResponse for (HttpStatusCode, T) where T : IntoResponse {
    fn into_response(self) -> HttpResponse {
        let (status, other) = self;
        let response = other.into_response();
        HttpResponse {
            status_code: status,
            headers: response.headers,
            body: response.body
        }
    }
}