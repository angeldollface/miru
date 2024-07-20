use serde::Serialize;
use super::requests::Request;
use super::responses::Response;
use super::requests::HTTPMethod;

pub struct Service<T: Serialize>{
    pub route: String,
    pub method: HTTPMethod,
    pub service: Box<dyn Fn(Request) -> Response<T>>
}

impl<T: Serialize> Service<T> {
    pub fn new(
        route: &String, 
        method: &HTTPMethod, 
        service: Box<dyn Fn(Request) -> Response<T>>
    ) -> Service<T> {
        Service { 
            route: route.to_owned(), 
            method: method.to_owned(), 
            service: service 
        }        
    }
}

pub struct App<T: Serialize> {
    pub host: String,
    pub port: u32,
    pub services: Vec<Service<T>>
}

impl<T: Serialize> App<T> {
    pub fn new(host: &String, port: &u32) -> App<T> {
        let mut services: Vec<Service<T>> = Vec::new();
        App {
            host: host.to_owned(),
            port: port.to_owned(),
            services: services
        }
    }
    pub fn add_service(
        &mut self, 
        route: &str, 
        method: &HTTPMethod, 
        service: Box<dyn Fn(Request) -> Response<T>>
    ) {
        &self.services.push(
            Service::new(&(route.to_string()), method, service)
        );
    }
}