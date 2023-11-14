use std::io::{prelude::*, BufReader};
use std::{env, thread};
use std::net::{TcpListener, TcpStream};


fn get_port() -> u16 {
    let mut port: u16 = 8080;
    match env::var("PING_LISTEN_PORT") {
        Ok(v) => port = v.parse().unwrap(),
        Err(_) => ()
    };
    return port
}

fn read_socket_request(mut stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    return request
}


fn make_response(status_code: i32, body: &str, content_type: &str) -> String {
    let mut response: String = "HTTP/1.1 ".to_owned();

    if status_code == 200 {
        response.push_str("200 OK\r\n");
    } else {
        response.push_str("404 Not Found\r\n");
    }

    response.push_str(format!("content-length: {}\r\n", body.len()).as_str());
    response.push_str(format!("content-type: {}\r\n", content_type).as_str());


    if body.len() != 0 {
        response.push_str("\r\n");
        response.push_str(&body);
    }

    return response
}

fn handle_client(mut stream: TcpStream) {
    let raw_request: Vec<String> = read_socket_request(&stream);
    let status_line: String = raw_request[0].clone();
    let status_parts: Vec<String> = status_line.split_whitespace().map(|s| s.to_string()).collect();
    
    if status_parts[0] != "GET" {
        stream.write_all(make_response(404, "", "application/json").as_bytes()).unwrap();
        return
    }

    if status_parts[1] != "/ping" {
        stream.write_all(make_response(404, "", "application/json").as_bytes()).unwrap();
        return
    }

    let mut response_content: String = "{".to_owned();

    let headers: Vec<String> = raw_request.clone().drain(1..).collect();
    for (index, header) in headers.iter().enumerate() {
        let header_parts: Vec<&str> = header.split(": ").collect();
        let mut current_header_object = format!("'{}': {:?}", header_parts[0], header_parts[1]);
        
        if index != headers.len() - 1 {
            current_header_object.push_str(", ");
        }
        response_content.push_str(current_header_object.as_str());
    }

    response_content.push_str("}");

    stream.write_all(make_response(200, &response_content, "application/json").as_bytes()).unwrap();
}

fn main() -> std::io::Result<()> {
    let port = get_port();
    println!("Starting server on  0.0.0.0:{}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_client(stream);
        });
    }
    Ok(())
}
