use super::err::MiruErr;
use std::collections::HashMap;

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
    pub headers: HashMap<String, String>
}

impl RequestHeaders{
    pub fn to_string(&self) -> String {
        todo!("Implement!")
    }
}

#[derive(Clone, Debug)]
pub struct RequestBody {
    pub body: String
}

impl RequestBody{
    pub fn to_string(&self) -> String {
        format!("{}", &self.body)
    }
}

pub fn parse_body(body: &String) -> Result<RequestBody, MiruErr> {
    if body.is_empty(){
        let e: String = "Empty body received.".to_string();
        Err::<RequestBody, MiruErr>(MiruErr::new(&e.to_string()))
    }
    else {
        Ok(RequestBody{ body: body.to_owned() })
    }
}

pub fn parse_headers(headers: &String) -> Result<RequestHeaders, MiruErr> {
    let mut header_lines: Vec<&str> = headers.split("\n").collect();
    if header_lines.is_empty(){
        let e: String = "Invalid request received.".to_string();
        Err::<RequestHeaders, MiruErr>(MiruErr::new(&e.to_string()))
    }
    else {
        let mut method: HTTPMethod = HTTPMethod::GET;
        let head: String = header_lines[0].to_string();
        let head_parts: Vec<&str> = head.split(" ").collect();
        if head_parts.len() != 3 {
            let e: String = format!("Error with header's head.");
            Err::<RequestHeaders, MiruErr>(MiruErr::new(&e.to_string()))
        }
        else {
            match head_parts[0] {
                "GET" => method = HTTPMethod::GET,
                "POST" => method = HTTPMethod::POST,
                "HEAD" => method = HTTPMethod::HEAD,
                "PUT" => method = HTTPMethod::PUT,
                "DELETE" => method = HTTPMethod::DELETE,
                "CONNECT" => method = HTTPMethod::CONNECT,
                "OPTIONS" => method = HTTPMethod::OPTIONS,
                "TRACE" => method = HTTPMethod::TRACE,
                "PATCH" => method = HTTPMethod::PATCH,
                _ => ()
            }
            let route: String = head_parts[1].to_string();
            let http_version: Vec<&str> = head_parts[2].split("/").collect();
            let mut headers: HashMap<String, String> = HashMap::new();
            header_lines.remove(0);
            for line in header_lines {
                let line_parts: Vec<&str> = line.split(":").collect();
                if line_parts.len() == 2 {
                    let key: String = line_parts[0].to_string();
                    let value: String = line_parts[1].to_string();
                    headers.insert(key, value);
                }
                else {
                    let e: String = format!("Error with header line: \"{}\"", line);
                    return Err::<RequestHeaders, MiruErr>(MiruErr::new(&e.to_string()))
                }
            }
            let req_headers: RequestHeaders = RequestHeaders{ 
                method: method,
                route: route,
                http_version: (http_version[1]).to_string(),
                headers: headers
            };
            Ok(req_headers)
        }
    }
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