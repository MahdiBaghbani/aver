use poem::http::StatusCode;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct PanicHandler {}

impl PanicHandler {
    pub fn new() -> Self {
        PanicHandler {}
    }
}

impl poem::middleware::PanicHandler for PanicHandler {
    type Response = (StatusCode, &'static str);

    fn get_response(&self, _err: Box<dyn Any + Send + 'static>) -> Self::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    }
}
