use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

struct HTTPRequest {
    method: HttpMethod,
    path: String,
    protocol_version: String,
    length: Option<i32>,
    body: String,
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

fn main() {
    let port = 7878;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("connection tried...");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut request = [0; 512];
    stream.read(&mut request).unwrap();
    let http_request = get_info_from_request(&String::from(
        String::from_utf8_lossy(&request[..]).get(..).unwrap(),
    ));
    match http_request.method {
        HttpMethod::GET => get_method(stream, &http_request),
        HttpMethod::POST => post_method(stream, &http_request),
        HttpMethod::PUT => put_method(),
        HttpMethod::PATCH => patch_method(),
        HttpMethod::DELETE => delete_method(),
    }
}

fn get_info_from_request(request: &String) -> HTTPRequest {
    println!(
        "REQUEST : **************************************************\n
{}\n*********************************************",
        request
    );

    let mut http_request = request.lines();
    let mut http_header = http_request.next().unwrap().split_whitespace();
    let mut length = Option::None;
    let mut body = String::new();

    for iter in http_request {
        let mut line = iter.split_whitespace();

        if length != Option::None {
            if iter.len() != 0 {
                body = format!("{}\r\n{}", body, iter);
            }
        }
        if line.next().unwrap_or_default() == "Content-Length:" {
            length = Option::Some(line.next().unwrap_or_default().parse().unwrap());
        }
    }
    let body = body.trim_matches(char::from(0)).to_string();

    let http_request = HTTPRequest {
        method: match http_header.next().unwrap() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            _ => panic!("Not Restful Api..."),
        },
        path: String::from(http_header.next().unwrap()),
        protocol_version: String::from(http_header.next().unwrap()),
        length: length,
        body: body,
    };
    http_request
}

fn get_method(mut stream: TcpStream, http_request: &HTTPRequest) {
    println!("GET METHOD {}", http_request.protocol_version);
    println!("url : {}", http_request.path);
    let filename = if http_request.path == "/" {
        "index.html"
    } else {
        &http_request.path[1..]
    };
    let mut contents = String::new();
    let (status_line, mut file) = match File::open(filename) {
        Result::Ok(file) => (
            format!("{} 200 OK\r\n\r\n", http_request.protocol_version),
            file,
        ),
        Result::Err(_e) => (
            format!("{} 404 NOT FOUND\r\n\r\n", http_request.protocol_version),
            File::open("404.html").unwrap(),
        ),
    };
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn post_method(mut stream: TcpStream, http_request: &HTTPRequest) {
    println!("POST METHOD {}", http_request.protocol_version);
    println!("url : {}", http_request.path);

    let response = if http_request.path == "/add_data" {
        let filename = "data.txt";
        post_method_api_adddata(filename, &http_request.body);
        format!("{} 200 OK\r\n\r\n", http_request.protocol_version)
    } else {
        format!("{} 400 BAD REQUEST\r\n\r\n", http_request.protocol_version)
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn post_method_api_adddata(filename: &str, data: &String) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)
        .unwrap();

    file.write(data.as_bytes()).unwrap();
}

fn put_method() {
    panic!("no implement...!");
}
fn patch_method() {
    panic!("no implement...!");
}
fn delete_method() {
    panic!("no implement...!");
}
