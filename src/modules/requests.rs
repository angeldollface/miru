use super::err::MiruErr;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Request {
    pub headers: RequestHeaders,
    pub body: Option<RequestBody>
}

impl Request {
    pub fn new(
        headers: &RequestHeaders, 
        body: &Option<RequestBody>
    ) -> Request {
        Request { headers: headers.to_owned(), body: body.to_owned() }
    }
    pub fn to_string(&self) -> String {
        match &self.body{
            Some(body) => format!(
                "Headers: {}\nBody: {}", 
                &self.headers.to_string(), 
                &body.to_string()
            ),
            None => format!(
                "Headers: {}", 
                &self.headers.to_string(), 
            )
        }
        
    }
}

#[derive(Clone, Debug)]
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

impl RequestHeaders{
    pub fn to_string(&self) -> String {
        todo!("Implement!")
    }
}

#[derive(Clone, Debug)]
pub struct RequestBody {
}

impl RequestBody{
    pub fn to_string(&self) -> String {
        todo!("Implement!")
    }
}

pub fn parse_body(body: &String) -> Result<RequestBody, MiruErr> {
    todo!("Implement parsing");
}

pub fn parse_headers(headers: &String) -> Result<RequestHeaders, MiruErr> {
    todo!("Implement parsing");
}

pub fn parse_request(request: &String) -> Result<Request, MiruErr> {
    let sep: String = format!("\r\n\r\n");
    let result: Request;
    if request.contains("\r\n\r\n"){
        let request_parts = request.split(&sep).collect::<Vec<&str>>();
        let header_portion: String = request_parts[0].to_string();
        let body_portion: String = request_parts[1].to_string();
        let headers: RequestHeaders = match parse_headers(&header_portion){
            Ok(headers) => headers,
            Err(e) => return Err::<Request, MiruErr>(MiruErr::new(&e.to_string()))
        };
        let body: RequestBody = match parse_body(&body_portion){
            Ok(headers) => headers,
            Err(e) => return Err::<Request, MiruErr>(MiruErr::new(&e.to_string()))
        };
        result = Request::new(&headers, &Some(body));
    }
    else {
        let request_parts = request.split(&sep).collect::<Vec<&str>>();
        let header_portion: String = request_parts[0].to_string();
        let headers: RequestHeaders = match parse_headers(&header_portion){
            Ok(headers) => headers,
            Err(e) => return Err::<Request, MiruErr>(MiruErr::new(&e.to_string()))
        };
        result = Request::new(&headers, &None);

    }
    Ok(result)
}