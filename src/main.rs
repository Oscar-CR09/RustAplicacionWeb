use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    // iniciar un servidor 
    let address="127.0.0.1:8081";
    let listener =TcpListener::bind(&address).unwrap();    
    println!("Servidor iniciado {}", &address);
    // escuchar por conexiones 
    for stream in listener.incoming(){
        let stream= stream.unwrap();
        println!("Stream recibido");

        handle_connection(stream);
    }
 
    
  
}

   //manejar estas conexiones 
fn handle_connection(mut stream:TcpStream){
    let mut buffer=[0;1024];

    stream.read(&mut buffer).unwrap();

    println!("Peticion recibida!");
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get =b"GET / HTTP/1.1\r\n";//127.0.0.1:8081

    if buffer.starts_with(get) {
        
        // responder al cliente
        send_index(stream);

    }else{
        send_not_found(stream);
    }
}

fn send_index(mut stream: TcpStream) {
    // Intentamos leer el archivo. Usamos match en lugar de unwrap
    let (status_line, contents) = match fs::read_to_string("index.html") {
        Ok(c) => ("HTTP/1.1 200 OK", c),
        Err(_) => (
            "HTTP/1.1 404 NOT FOUND",
            String::from("<!DOCTYPE html><html><body><h1>404 - Archivo no encontrado</h1><p>Asegurate de crear index.html en la carpeta correcta.</p></body></html>")
        ),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/* 
fn send_to_client(mut stream:TcpStream){

    let contents=fs::read_to_string("index.html").unwrap();
    
    let response=format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    contents.len(),
    contents
    ); // format carriege return. \r
    //line feed LF - CRLF \n

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    }
*/


