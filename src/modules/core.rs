/*
MIRU by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

use std::io::BufReader;
use crate::ThreadPool;

use super::err::MiruErr;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::read_to_string;
use super::requests::Request;
use super::requests::parse_request;
use std::thread::available_parallelism;

pub fn run_server(ip: &str, port: &str) -> Result<(), MiruErr> {
    let bind_string: String = format!("{}:{}", ip, port);
    let listener = match TcpListener::bind(bind_string){
        Ok(listener) => listener,
        Err(e) => return Err::<(), MiruErr>(MiruErr::new(&e.to_string()))
    };
    let streams = listener.incoming();
    for stream in streams {
        let stream: TcpStream = match stream{
            Ok(stream) => stream,
            Err(e) => return Err::<(), MiruErr>(MiruErr::new(&e.to_string()))
        };
        let max_thread_num: usize = match available_parallelism(){
            Ok(max_thread_num) => max_thread_num.get(),
            Err(e) => return Err::<(), MiruErr>(MiruErr::new(&e.to_string()))
        };
        let pool: ThreadPool = ThreadPool::new(max_thread_num).unwrap();
        pool.execute(
            ||{
                let _request: Request = handle_connection(stream).unwrap();
            }
        );
    }
    Ok(())
}

pub fn handle_connection(stream: TcpStream) -> Result<Request, MiruErr>{
    let mut buf_reader = BufReader::new(stream);
    let request_string: String = match read_to_string(&mut buf_reader){
        Ok(request_string) => request_string,
        Err(e) => return Err::<Request, MiruErr>(MiruErr::new(&e.to_string()))
    };
    let parsed: Request = match parse_request(&request_string){
        Ok(parsed) => parsed,
        Err(e) => return Err::<Request, MiruErr>(MiruErr::new(&e.to_string()))
    };
    Ok(parsed)
}