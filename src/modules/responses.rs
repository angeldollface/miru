use serde::Serialize;

pub struct Response<T: Serialize> {
    pub payload: T
}

impl<T: Serialize> Response<T> {
    pub fn new(payload: T) -> Response<T> {
        Response { payload: payload }
    }
}