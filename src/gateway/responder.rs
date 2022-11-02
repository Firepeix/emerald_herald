use std::io::Cursor;

use rocket::{http::{ContentType, Status}, response::Responder, Response};

pub struct ProxiedResponder {
    body: String,
    status: Status,
}

impl ProxiedResponder {
    pub fn new(body: String, status: Status) -> Self {
        ProxiedResponder { body, status }
    }
}

#[rocket::async_trait]
impl<'r, 'o: 'r> Responder<'r, 'o> for ProxiedResponder {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        Response::build()
            .header(ContentType::Plain)
            .status(self.status)
            .sized_body(self.body.len(), Cursor::new(self.body))
            .ok()
    }
}

