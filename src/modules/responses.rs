use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Response<T: Serialize + DeserializeOwned> {
    pub payload: T
}

impl<T: Serialize + DeserializeOwned> Response<T> {
    pub fn new(payload: T) -> Response<T> {
        Response { payload: payload }
    }
}