use super::err::MiruErr;

pub enum HTTPMethod {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

pub struct Request {
    pub headers: RequestHeaders,
    pub body: RequestBody
}

pub struct RequestHeaders {
    pub method: HTTPMethod,
    pub route: String,
    pub http_version: String,
    pub host: String,
    pub user_agent: String,
    pub accept_formats: Vec<String>,
    pub accept_language: String,
    pub accept_encoding: String,
    pub connection: String,
    pub cookie: String,
    pub body: String,
}

pub struct RequestBody {

}

pub fn parse_body(body: &String) -> Result<RequestBody, MiruErr> {
    todo!("Implement parsing");
}

pub fn parse_headers(headers: &String) -> Result<RequestHeaders, MiruErr> {
    todo!("Implement parsing");
}

pub fn parse_request(request: &String) -> Result<Request, MiruErr> {
    let sep: String = format!("\r\n\r\n");
    if request.contains("\r\n\r\n"){
        let request_parts = request.split(&sep).collect::<Vec<&str>>();
        let header_portion: String = request_parts[0].to_string();
        let body_portion: String = request_parts[1].to_string();
    }
    else {

    }
    let headers: RequestHeaders = match parse_headers(header_portion){
        Ok(headers) => headers,
        Err(e) => return Err::<Request, MiruErr>(MiruErr::new(&e.to_string()))
    }
    let body: RequestBody = match parse_headers(header_portion){
        Ok(headers) => headers,
        Err(e) => return Err::<Request, MiruErr>(MiruErr::new(&e.to_string()))
    }:
}