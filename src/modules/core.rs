/*
MIRU by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

use std::thread::spawn;
use std::io::BufReader;
use super::err::MiruErr;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::read_to_string;
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
        spawn(
            ||{
                let request: String = match handle_connection(stream){
                    Ok(request) => request,
                    Err(e) => return Err::<(), MiruErr>(MiruErr::new(&e.to_string()))
                };
                println!("{}", request);
                Ok(())
            }
        );
    }
    Ok(())
}

pub fn handle_connection(stream: TcpStream) -> Result<String, MiruErr>{
    let mut buf_reader = BufReader::new(stream);
    let request_string: String = match read_to_string(&mut buf_reader){
        Ok(request_string) => request_string,
        Err(e) => return Err::<String, MiruErr>(MiruErr::new(&e.to_string()))
    };
    Ok(request_string)
}